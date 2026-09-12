//! Mirrors `net.h4bbo.lisbon.messages.types.*`.

use crate::game::player::Player;
use crate::server::netty::streams::{NettyRequest, NettyResponse};

/// Mirrors the `MessageComposer` abstract class.
///
/// An outgoing message knows how to compose itself into a [`NettyResponse`]
/// and reports its protocol header.
pub trait MessageComposer {
    /// Write the message to send back to the client.
    fn compose(&self, response: &mut NettyResponse);

    /// Get the header.
    fn get_header(&self) -> i16;
}

/// Mirrors the `MessageEvent` interface.
///
/// An incoming message handler reacts to a protocol frame.
pub trait MessageEvent {
    /// The handler's simple type name (the Java `getSimpleName()`).
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>().rsplit("::").next().unwrap_or("")
    }

    /// Handle the incoming client message. The dispatcher always enters via
    /// `handle_mut`; this default is only reached when a handler that
    /// requires exclusive access is invoked directly with a shared reference.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Err("handler requires exclusive player access".to_string())
    }

    /// Handle the incoming client message with exclusive access to the
    /// player. The connection dispatcher holds the connection's
    /// `Mutex<Player>` while invoking this, mirroring the Java handler,
    /// which receives the `Player` directly.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        self.handle(player, reader)
    }
}
