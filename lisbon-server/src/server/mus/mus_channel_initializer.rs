//! Mirrors `net.h4bbo.lisbon.server.mus.MusChannelInitializer`.
//!
//! The Java `ChannelInitializer` installs a Netty `ChannelPipeline`. In tokio
//! there is no per-connection pipeline; the equivalent is the
//! `MusConnectionHandler` state machine, which preserves the handler order:
//! `frameDecoder` (folding into `MusNetworkDecoder`) → `gameDecoder`
//! (`MusNetworkDecoder`) → `gameEncoder` (`MusNetworkEncoder`) → `handler`
//! (`MusConnectionHandler::handle_message`).

/// Initialises the per-connection "pipeline".
pub struct MusChannelInitializer;

impl MusChannelInitializer {
    /// Mirrors the `MusChannelInitializer(MusServer)` constructor.
    pub fn new() -> Self {
        Self
    }

    /// Mirrors `initChannel(SocketChannel)`.
    ///
    /// Port note: no Netty `ChannelPipeline`; the ordering
    /// `frameDecoder → gameDecoder → gameEncoder → handler` is preserved inside
    /// `MusConnectionHandler::handle` (decode frame → decode message → encode
    /// reply → write).
    pub fn init_channel(&self) {
        // The per-connection state machine is the pipeline.
    }
}
