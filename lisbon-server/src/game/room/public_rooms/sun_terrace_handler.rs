//! Mirrors `net.h4bbo.lisbon.game.room.public_rooms.SunTerraceHandler`.
use crate::game::room::entities::room_entity::RoomEntity;

pub struct SunTerraceHandler;

impl SunTerraceHandler {
    /// Mirrors `isRedirected(RoomEntity, int, int)` (the Java
    // `room.getModel()` NPE is an early `false` return here; the Java
    // `getTile` NPE is a `0.0` height here).
    pub fn is_redirected(room_entity: &RoomEntity, target_x: i32, target_y: i32) -> bool {
        let Some(room) = room_entity.get_room() else {
            return false;
        };

        let Some(model) = room.get_model() else {
            return false;
        };

        if model.get_name() != "sun_terrace" {
            return false;
        }

        let current_z = room_entity.get_position().get_z();
        let goal_z = room
            .get_mapping().lock()
            .get_tile(&room, target_x, target_y)
            .map(|tile| tile.get_tile_height())
            .unwrap_or(0.0);

        if (current_z < 8.0)
            && goal_z >= 8.0
            && room_entity.get_position().get_x() != 4
            && room_entity.get_position().get_y() != 18
        {
            return true;
        }

        target_x == 4
            && target_y == 18
            && room_entity.get_position().get_x() != 6
            && room_entity.get_position().get_y() != 21
    }
}
