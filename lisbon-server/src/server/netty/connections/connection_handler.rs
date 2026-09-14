//! Mirrors `net.h4bbo.lisbon.server.netty.connections.ConnectionHandler`.
//!
//! The Java `SimpleChannelInboundHandler<NettyRequest>` becomes a per-connection
//! async task: `channelRegistered` / `channelUnregistered` bracket the read
//! loop, `channelRead0` is the decode + `MessageHandler` dispatch, and the
//! `IdleStateHandler(60, 0, 0)` read-idle event maps onto a 60-second read
//! timeout that invokes `IdleConnectionHandler::on_read_idle`.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use std::net::SocketAddr;

use parking_lot::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::lisbon::Lisbon;
use crate::log::Log;
use crate::messages::message_handler::MessageHandler;
use crate::messages::outgoing::handshake::hello::HELLO;
use crate::server::netty::codec::inbound_crypto_decoder::InboundCryptoDecoder;
use crate::server::netty::codec::network_decoder::NetworkDecoder;
use crate::server::netty::codec::outbound_crypto_encoder::OutboundCryptoEncoder;
use crate::server::netty::connections::idle_connection_handler::IdleConnectionHandler;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;
use crate::server::netty::netty_server::ConnectionRecord;
use crate::server::netty::streams::netty_request::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

/// Per-connection game handler (the tokio equivalent of the inbound handler).
pub struct ConnectionHandler {
    channels: Arc<Mutex<Vec<ConnectionRecord>>>,
    connection_ids: Arc<AtomicI64>,
}

impl ConnectionHandler {
    /// Create a handler for one accepted connection.
    pub fn new(
        channels: Arc<Mutex<Vec<ConnectionRecord>>>,
        connection_ids: Arc<AtomicI64>,
    ) -> Self {
        Self {
            channels,
            connection_ids,
        }
    }

    /// Run the per-connection state machine.
    ///
    /// Mirrors `channelRegistered` → `channelRead0` (loop) →
    /// `channelUnregistered`.
    pub async fn handle(self, stream: TcpStream, peer: SocketAddr, connection_id: i64) {
        let ip = peer.ip().to_string();

        // channelRegistered — per-IP connection limit.
        let max_connections_per_ip =
            GameConfiguration::get_instance().get_integer("max.connections.per.ip");
        // The Java source itself carries a TODO here (IP ban checking).
        if max_connections_per_ip > 0 {
            let count = self
                .channels
                .lock()
                .iter()
                .filter(|record| record.ip == ip)
                .count() as i32;
            if count >= max_connections_per_ip {
                tracing::info!(
                    "Kicking off connection from {} to make room for new connection",
                    ip
                );
                return;
            }
        }

        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let local_port = stream
            .local_addr()
            .map(|a| a.port() as i32)
            .unwrap_or(0);
        let network =
            NettyPlayerNetwork::new(connection_id, ip.clone(), local_port, write_tx);
        let player = Arc::new(Mutex::new(Player::new(network)));

        // Register in the channel group.
        self.channels
            .lock()
            .push(ConnectionRecord { id: connection_id, ip: ip.clone() });
        if Lisbon::is_shutting_down() {
            Log::get_error_logger()
                .error(format!("Could not accept connection from {}", ip));
            self.channels.lock().retain(|c| c.id != connection_id);
            self.connection_ids.fetch_sub(1, Ordering::Relaxed);
            return;
        }

        // Greeting.
        {
            let player_guard = player.lock();
            player_guard.send(&HELLO);
        }
        tracing::info!("[{}] Connection from {}", connection_id, ip);

        let (mut read, mut write) = stream.into_split();

        // Outbound write task (mirrors `channel.writeAndFlush(...)`; the
        // `OutboundCryptoEncoder` pipeline stage is the conditional encrypt
        // below).
        let write_player = player.clone();
        let write_task = tokio::spawn(async move {
            while let Some(frame) = write_rx.recv().await {
                let frame = {
                    let mut player_guard = write_player.lock();
                    if player_guard.is_outbound_encrypted() {
                        OutboundCryptoEncoder::encrypt_frame(&mut player_guard, &frame)
                    } else {
                        frame
                    }
                };
                let _ = write.write_all(&frame).await;
                let _ = write.flush().await;
            }
        });

        // channelRead0 (loop) with a 60-second read-idle timeout.
        let mut buf: Vec<u8> = Vec::new();
        let mut chunk = vec![0u8; 65536];
        loop {
            match tokio::time::timeout(Duration::from_secs(60), read.read(&mut chunk)).await {
                Ok(Ok(0)) => break,
                Ok(Ok(n)) => buf.extend_from_slice(&chunk[..n]),
                Ok(Err(error)) => {
                    Self::on_error(&error);
                    break;
                }
                Err(_timeout) => {
                    // ReADER_IDLE event.
                    let mut player_guard = player.lock();
                    IdleConnectionHandler::on_read_idle(&mut player_guard);
                    drop(player_guard);
                    if player.lock().get_network().is_closed() {
                        break;
                    }
                }
            }

            // Decode frames and dispatch them.
            loop {
                // Phase 1: decode (mutable lock; dropped before the dispatch lock).
                let request: Option<NettyRequest> = {
                    let mut player_guard = player.lock();
                    let inbound_crypto = player_guard.is_inbound_encrypted()
                        && player_guard.get_inbound_cipher().is_some();
                    if inbound_crypto {
                        match InboundCryptoDecoder::decode(&mut *player_guard, &mut buf) {
                            Some(mut plaintext) => NetworkDecoder::decode(&mut plaintext),
                            None => None,
                        }
                    } else {
                        NetworkDecoder::decode(&mut buf)
                    }
                };

                match request {
                    Some(mut request) => {
                        // Phase 2: dispatch (a separate, non-re-entrant lock).
                        // The handler chain reaches the DAOs, which `block_on`
                        // the dedicated storage runtime; that panics when the
                        // current thread is an async worker, so run it on the
                        // blocking pool (a blocking-pool thread is not a
                        // runtime worker thread).
                        let player = player.clone();
                        let _ = tokio::task::spawn_blocking(move || {
                            let mut player_guard = player.lock();
                            MessageHandler::get_instance()
                                .handle_request(&mut *player_guard, &mut request);
                        })
                        .await;
                    }
                    None => break,
                }
            }
        }

        // channelUnregistered
        self.connection_ids.fetch_sub(1, Ordering::Relaxed);
        self.channels.lock().retain(|c| c.id != connection_id);

        {
            let mut player_guard = player.lock();
            player_guard.dispose();
        }
        tracing::info!("[{}] Disconnection from {}", connection_id, ip);

        write_task.abort();
    }

    /// Mirrors `exceptionCaught(ChannelHandlerContext, Throwable)`.
    ///
    /// The Java handler logs non-`IOException` errors; tokio surfaces read
    /// errors here, and the read loop closes the connection.
    fn on_error(error: &std::io::Error) {
        Log::get_error_logger()
            .error_with("Netty error occurred", error.to_string());
    }
}
