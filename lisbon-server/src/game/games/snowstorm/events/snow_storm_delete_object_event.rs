//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormDeleteObjectEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormDeleteObjectEvent {
    object_id: i32,
}

impl SnowStormDeleteObjectEvent {
    /// Mirrors the `SnowStormDeleteObjectEvent(int)` constructor.
    pub fn new(object_id: i32) -> Self {
        Self { object_id }
    }
}

impl GameObject for SnowStormDeleteObjectEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.object_id);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarRemoveObjectEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
