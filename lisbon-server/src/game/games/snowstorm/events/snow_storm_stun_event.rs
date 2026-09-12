//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormStunEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormStunEvent {
    stunned_id: i32,
    thrower_id: i32,
    hit_direction: i32,
}

impl SnowStormStunEvent {
    /// Mirrors the `SnowStormStunEvent(int, int, int)` constructor (the
    /// Java assigns `stunnedId = throwerId` and `throwerId = hitId`).
    pub fn new(stunned_id: i32, thrower_id: i32, hit_direction: i32) -> Self {
        Self {
            stunned_id,
            thrower_id,
            hit_direction,
        }
    }
}

impl GameObject for SnowStormStunEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.stunned_id);
        response.write_int(self.thrower_id);
        response.write_int(self.hit_direction);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarStunEvent
    }

    /// Mirrors `getId()`.
    // Port note: the Java constructor passes `-1` to `super`.
    fn get_id(&self) -> i32 {
        -1
    }
}
