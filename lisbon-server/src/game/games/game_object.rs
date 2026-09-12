//! Mirrors `net.h4bbo.lisbon.game.games.GameObject`.
// Port note: the Java abstract class carries `id` / `gameObjectType`
// fields; as a trait the state is held by the implementors.

use std::any::Any;

use crate::game::games::enums::game_object_type::GameObjectType;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the abstract `GameObject` class (the `Any` supertrait allows
// downcasting the boxed objects; `Send + Sync` lets boxed objects live inside
// the `Arc<Mutex<GamePlayer>>` shared across threads).
pub trait GameObject: Any + Send + Sync {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse);

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType;

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32;
}
