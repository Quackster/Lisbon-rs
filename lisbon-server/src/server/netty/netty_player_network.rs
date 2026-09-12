//! Mirrors `net.h4bbo.lisbon.server.netty.NettyPlayerNetwork`.
//!
//! The Netty `Channel` is represented by an outbound `mpsc` sender (the
//! connection task owns the socket write half) plus the connection id, the
//! local port and the remote IP. `send` encodes a `MessageComposer` via
//! `NetworkEncoder` and pushes the frame to the write task.

use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::messages::types::MessageComposer;
use crate::server::netty::codec::network_encoder::NetworkEncoder;

/// The network handle for one connected player. The Java field is a Netty
/// `Channel`; here the outbound write channel + close flag stand in for it
/// (`getChannel()` is folded into `send` / `disconnect`).
pub struct NettyPlayerNetwork {
    connection_id: i64,
    remote_ip: String,
    local_port: i32,
    write_tx: UnboundedSender<Vec<u8>>,
    closed: Arc<AtomicBool>,
}

impl NettyPlayerNetwork {
    /// Mirrors the `NettyPlayerNetwork(Channel, int)` constructor.
    pub fn new(
        connection_id: i64,
        remote_ip: String,
        local_port: i32,
        write_tx: UnboundedSender<Vec<u8>>,
    ) -> Self {
        Self {
            connection_id,
            remote_ip,
            local_port,
            write_tx,
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Mirrors `getConnectionId()`.
    pub fn get_connection_id(&self) -> i64 {
        self.connection_id
    }

    /// Mirrors `getPort()`.
    pub fn get_port(&self) -> i32 {
        self.local_port
    }

    /// Mirrors `send(Object)` (`channel.writeAndFlush(response)`).
    pub fn send(&self, response: &dyn MessageComposer) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        if let Some(frame) = NetworkEncoder::encode_composer(response) {
            let _ = self.write_tx.send(frame);
        }
    }

    /// Mirrors `send(Object)` for the non-`MessageComposer` branches: the
    /// encoder's `String` branch writes the raw bytes; any other object type
    /// is dropped (the Java encoder's `instanceof` check yields no bytes for
    /// it either).
    pub fn send_object(&self, object: &dyn Any) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        if let Some(value) = object.downcast_ref::<String>() {
            let frame = NetworkEncoder::encode_string(value);
            let _ = self.write_tx.send(frame);
        }
    }

    /// Mirrors `sendQueued(MessageComposer)` (`channel.write(response)`).
    /// The queue-vs-immediate distinction is not preserved; the write task
    /// flushes after every frame.
    pub fn send_queued(&self, response: &dyn MessageComposer) {
        self.send(response)
    }

    /// Mirrors `flush()` (`channel.flush()`).
    /// The write task flushes after each frame; an explicit flush is a no-op.
    pub fn flush(&self) {}

    /// Mirrors `disconnect()` (`channel.close()`).
    pub fn disconnect(&self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    /// Whether the connection has been marked for close.
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    /// Mirrors `getIpAddress(Channel)` (static in Java).
    pub fn get_ip_address(network: &NettyPlayerNetwork) -> String {
        network.remote_ip.clone()
    }

    /// The remote IP of the connection.
    pub fn get_remote_ip(&self) -> &str {
        &self.remote_ip
    }
}
