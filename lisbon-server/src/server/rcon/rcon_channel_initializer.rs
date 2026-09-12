//! Mirrors `net.h4bbo.lisbon.server.rcon.RconChannelInitializer`.
//!
//! The Java `ChannelInitializer` installs a Netty `ChannelPipeline`
//! (`gameDecoder` → `RconNetworkDecoder`, `handler` → `RconConnectionHandler`).
//! In tokio there is no per-connection pipeline; the equivalent is the
//! `RconConnectionHandler` state machine, which preserves the handler order:
//! `gameDecoder` (folding into `RconNetworkDecoder`) → `handler`
//! (`RconConnectionHandler::handle_message`).

/// Initialises the per-connection "pipeline".
pub struct RconChannelInitializer;

impl RconChannelInitializer {
    /// Mirrors the `RconChannelInitializer(RconServer)` constructor.
    pub fn new() -> Self {
        Self
    }

    /// Mirrors `initChannel(SocketChannel)`.
    ///
    /// Port note: no Netty `ChannelPipeline`; the ordering
    /// `gameDecoder → handler` is preserved inside
    /// `RconConnectionHandler::handle` (decode frame → handle message).
    pub fn init_channel(&self) {
        // The per-connection state machine is the pipeline.
    }
}
