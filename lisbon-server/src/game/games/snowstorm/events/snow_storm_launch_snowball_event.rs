//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormLaunchSnowballEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormLaunchSnowballEvent {
    object_id: i32,
    thrower_id: i32,
    x: i32,
    y: i32,
    trajectory: i32,
}

impl SnowStormLaunchSnowballEvent {
    /// Mirrors the `SnowStormLaunchSnowballEvent(int, int, int, int, int)` constructor.
    pub fn new(
        object_id: i32,
        thrower_id: i32,
        x: i32,
        y: i32,
        trajectory: i32,
    ) -> Self {
        Self {
            object_id,
            thrower_id,
            x,
            y,
            trajectory,
        }
    }
}

impl GameObject for SnowStormLaunchSnowballEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_game_object_type().get_object_id());
        response.write_int(self.object_id);
        response.write_int(self.thrower_id);
        response.write_int(self.x);
        response.write_int(self.y);
        response.write_int(self.trajectory);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarThrowEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
