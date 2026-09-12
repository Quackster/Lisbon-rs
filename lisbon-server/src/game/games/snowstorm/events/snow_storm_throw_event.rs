//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormThrowEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormThrowEvent {
    object_id: i32,
    x: i32,
    y: i32,
    throw_height: i32,
}

impl SnowStormThrowEvent {
    /// Mirrors the `SnowStormThrowEvent(int, int, int, int)` constructor.
    pub fn new(object_id: i32, x: i32, y: i32, throw_height: i32) -> Self {
        Self {
            object_id,
            x,
            y,
            throw_height,
        }
    }
}

impl GameObject for SnowStormThrowEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.object_id);
        response.write_int(self.x);
        response.write_int(self.y);
        response.write_int(self.throw_height);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarTargetThrowEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
