//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormMachineMoveSnowballsEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormMachineMoveSnowballsEvent {
    player_id: i32,
    machine_id: i32,
}

impl SnowStormMachineMoveSnowballsEvent {
    /// Mirrors the `SnowStormMachineMoveSnowballsEvent(int, int)` constructor.
    pub fn new(player_id: i32, machine_id: i32) -> Self {
        Self {
            player_id,
            machine_id,
        }
    }
}

impl GameObject for SnowStormMachineMoveSnowballsEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.player_id);
        response.write_int(self.machine_id);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarMachineMoveSnowballsEvent
    }

    /// Mirrors `getId()`.
    // Port note: the Java constructor passes `-1` to `super`.
    fn get_id(&self) -> i32 {
        -1
    }
}
