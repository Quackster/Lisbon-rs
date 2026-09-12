//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.events.SnowStormAvatarMoveEvent`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormAvatarMoveEvent {
    object_id: i32,
    x: i32,
    y: i32,
}

impl SnowStormAvatarMoveEvent {
    /// Mirrors the `SnowStormAvatarMoveEvent(int, int, int)` constructor.
    pub fn new(object_id: i32, x: i32, y: i32) -> Self {
        Self { object_id, x, y }
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl GameObject for SnowStormAvatarMoveEvent {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(GameObjectType::SnowwarAvatarMoveEvent.get_object_id());
        response.write_int(self.object_id);
        // Port note: the Java writes the raw `X` / `Y` (the
        // `convertToWorldCoordinate` call is commented out in the Java).
        response.write_int(self.x);
        response.write_int(self.y);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarAvatarMoveEvent
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
