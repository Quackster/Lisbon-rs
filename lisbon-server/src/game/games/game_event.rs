//! Mirrors `net.h4bbo.lisbon.game.games.GameEvent`.
// Port note: the Java abstract class carries a `gameEventType` field; as a
// trait the state is held by the implementors.

use std::any::Any;

use crate::game::games::enums::game_event_type::GameEventType;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the abstract `GameEvent` class (the `Any` supertrait allows
// downcasting the boxed events; `Send` lets the boxed events live inside the
// `Arc<Mutex<...>>` shared across threads).
pub trait GameEvent: Any + Send {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse);

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType;
}
