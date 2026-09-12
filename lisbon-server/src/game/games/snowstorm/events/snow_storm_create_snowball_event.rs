//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormCreateSnowballEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormCreateSnowballEvent {
    object_id: i32,
}

impl SnowStormCreateSnowballEvent {
    /// Mirrors the `SnowStormCreateSnowballEvent(int)` constructor.
    pub fn new(object_id: i32) -> Self {
        Self { object_id }
    }
}

impl GameObject for SnowStormCreateSnowballEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(GameObjectType::SnowwarCreateSnowballEvent.get_object_id());
        response.write_int(self.object_id);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarCreateSnowballEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
