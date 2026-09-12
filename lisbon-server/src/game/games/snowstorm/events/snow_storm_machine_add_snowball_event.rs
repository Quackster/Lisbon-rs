//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormMachineAddSnowballEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormMachineAddSnowballEvent {
    machine_id: i32,
}

impl SnowStormMachineAddSnowballEvent {
    /// Mirrors the `SnowStormMachineAddSnowballEvent(int)` constructor.
    pub fn new(machine_id: i32) -> Self {
        Self { machine_id }
    }
}

impl GameObject for SnowStormMachineAddSnowballEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.machine_id);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarMachineAddSnowballEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.machine_id
    }
}
