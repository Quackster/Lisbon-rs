//! Mirrors `net.h4bbo.lisbon.server.netty.NettyChannelInitializer`.
//!
//! The Java `ChannelInitializer` installs a Netty `ChannelPipeline`:
//! `gameEncoder` (`NetworkEncoder`) → `gameDecoder` (`NetworkDecoder`) →
//! `handler` (`ConnectionHandler`) → `idleStateHandler` (`IdleStateHandler`,
//! 60s read-idle) → `idleHandler` (`IdleConnectionHandler`). In tokio there is
//! no per-connection pipeline; the equivalent is the `ConnectionHandler` state
//! machine, which preserves that ordering (encode on write, decode + dispatch
//! on read, 60s read-idle → idle handler).

/// Initialises the per-connection "pipeline".
pub struct NettyChannelInitializer;

impl NettyChannelInitializer {
    /// Mirrors the `NettyChannelInitializer(NettyServer)` constructor.
    pub fn new() -> Self {
        Self
    }

    /// Mirrors `initChannel(SocketChannel)`.
    ///
    /// Port note: no Netty `ChannelPipeline`; the ordering
    /// `gameEncoder → gameDecoder → handler → idleStateHandler → idleHandler`
    /// is preserved inside `ConnectionHandler::handle` (encode on the write
    /// task, decode + dispatch in the read loop, 60s read timeout →
    /// `IdleConnectionHandler`).
    pub fn init_channel(&self) {
        // The per-connection state machine is the pipeline.
    }
}
