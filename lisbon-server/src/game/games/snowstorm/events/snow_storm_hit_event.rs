//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormHitEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormHitEvent {
    thrower_id: i32,
    target_id: i32,
    hit_direction: i32,
}

impl SnowStormHitEvent {
    /// Mirrors the `SnowStormHitEvent(int, int, int)` constructor.
    pub fn new(thrower_id: i32, target_id: i32, hit_direction: i32) -> Self {
        Self {
            thrower_id,
            target_id,
            hit_direction,
        }
    }
}

impl GameObject for SnowStormHitEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.thrower_id);
        response.write_int(self.target_id);
        response.write_int(self.hit_direction);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowstormHitEvent
    }

    /// Mirrors `getId()`.
    // Port note: the Java constructor passes `-1` to `super`.
    fn get_id(&self) -> i32 {
        -1
    }
}
