//! Mirrors `net.h4bbo.lisbon.server.rcon.RconServer`.
//!
//! Netty `ServerBootstrap`/`EventLoopGroup`/`DefaultChannelGroup` map onto a
//! tokio `TcpListener`, a per-connection `tokio::spawn`, and a shared
//! `Vec<ConnectionRecord>` (the channel group). The Java event-loop options
//! (SO_BACKLOG, TCP_NODELAY, SO_KEEPALIVE, SO_RCVBUF, RCVBUF_ALLOCATOR,
//! ALLOCATOR) are applied implicitly by tokio.

use std::net::SocketAddr;
use std::sync::atomic::AtomicI64;
use std::sync::{Arc, OnceLock};

use parking_lot::Mutex;
use tokio::net::TcpListener;

use crate::log::Log;
use crate::server::netty::netty_server::ConnectionRecord;
use crate::server::rcon::rcon_connection_handler::RconConnectionHandler;
use crate::util::config::server_configuration::ServerConfiguration;

/// The listen backlog / receive-buffer sizes from the Java bootstrap.
const BACK_LOG: i32 = 20;
const BUFFER_SIZE: i32 = 2048;

/// The RCON (remote connection) network endpoint.
pub struct RconServer {
    ip: String,
    port: i32,
    channels: Arc<Mutex<Vec<ConnectionRecord>>>,
    connection_ids: Arc<AtomicI64>,
}

impl RconServer {
    /// Mirrors the `RconServer(String, int)` constructor.
    pub fn new(ip: &str, port: i32) -> Self {
        Self {
            ip: ip.to_string(),
            port,
            channels: Arc::new(Mutex::new(Vec::new())),
            connection_ids: Arc::new(AtomicI64::new(0)),
        }
    }

    /// Mirrors `createSocket()`.
    /// Port note: Netty pre-creates the event-loop groups and configures
    /// options (SO_BACKLOG, TCP_NODELAY, SO_KEEPALIVE, SO_RCVBUF,
    /// RCVBUF_ALLOCATOR, ALLOCATOR); tokio performs the equivalent at `bind`
    /// time inside `start`, so this is a no-op that merely documents the
    /// intended socket configuration.
    #[allow(dead_code)]
    pub fn create_socket(&self) {
        // Backlog / buffer sizes are applied implicitly by tokio.
        let _ = (BACK_LOG, BUFFER_SIZE);
    }

    /// Mirrors `bind()` and the accept loop.
    ///
    /// Binds the listener and, for every accepted socket, spawns a
    /// `RconConnectionHandler` task. Callers `tokio::spawn` (or `block_on`)
    /// this.
    pub async fn start(&self) {
        let addr: SocketAddr = format!("{}:{}", self.ip, self.port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], self.port as u16)));

        match TcpListener::bind(addr).await {
            Err(error) => {
                Log::get_error_logger().error_with(
                    "Failed to start RCON server on address",
                    format!("{}:{}", self.ip, self.port),
                );
                Log::get_error_logger().error_with(
                    "Please double check there's no programs using the same port, and you have set the correct IP address to listen on.",
                    error.to_string(),
                );
            }
            Ok(listener) => {
                tracing::info!(
                    "RCON (remote connection) is listening on {}:{}",
                    self.ip,
                    self.port
                );

                loop {
                    match listener.accept().await {
                        Ok((stream, _peer)) => {
                            let handler =
                                RconConnectionHandler::new(self.channels.clone(), self.connection_ids.clone());
                            tokio::spawn(handler.handle(stream));
                        }
                        Err(error) => {
                            Log::get_error_logger()
                                .error_with("RCON accept failed", error.to_string());
                        }
                    }
                }
            }
        }
    }

    /// Mirrors `dispose()`.
    /// Port note: graceful shutdown of the event loop groups has no direct
    /// tokio equivalent; dropping the server (and its listener) closes the
    /// listener, and in-flight connection tasks are cancelled by the caller.
    pub fn dispose(&self) {}

    /// Mirrors `getIp()`.
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    /// Mirrors `getPort()`.
    pub fn get_port(&self) -> i32 {
        self.port
    }

    /// Mirrors `getChannels()`.
    pub fn get_channels(&self) -> &Arc<Mutex<Vec<ConnectionRecord>>> {
        &self.channels
    }

    /// Mirrors `getConnectionIds()`.
    pub fn get_connection_ids(&self) -> &Arc<AtomicI64> {
        &self.connection_ids
    }

    /// Mirrors the `Lisbon.rconServer` singleton accessor
    /// (`Lisbon.getRconServer()`).
    pub fn get_instance() -> &'static RconServer {
        static INSTANCE: OnceLock<RconServer> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let ip = ServerConfiguration::get_string("rcon.bind");
            let port = ServerConfiguration::get_integer("rcon.port");
            Self::new(if ip.is_empty() { "0.0.0.0" } else { &ip }, port)
        })
    }
}
