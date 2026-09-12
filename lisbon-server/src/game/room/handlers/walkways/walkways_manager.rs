//! Mirrors `net.h4bbo.lisbon.game.room.handlers.walkways.WalkwaysManager`.
use std::sync::OnceLock;

use crate::dao::mysql::public_rooms_dao::PublicRoomsDao;
use crate::game::pathfinder::position::Position;
use crate::game::room::handlers::walkways::walkways_entrance::WalkwaysEntrance;
use crate::game::room::room::Room;

pub struct WalkwaysManager {
    walkways: Vec<WalkwaysEntrance>,
}

impl WalkwaysManager {
    /// Get the instance.
    pub fn get_instance() -> &'static WalkwaysManager {
        static INSTANCE: OnceLock<WalkwaysManager> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            walkways: PublicRoomsDao::get_walkways(),
        })
    }

    /// Mirrors `createWalkway(int, int, String, String)`.
    pub fn create_walkway(
        room_id: i32,
        room_target_id: i32,
        from_coords: &str,
        destination: Option<&str>,
    ) -> WalkwaysEntrance {
        let mut coordinates: Vec<Position> = Vec::new();

        for coord in from_coords.split(' ') {
            let parts: Vec<&str> = coord.split(',').collect();

            if let (Some(x), Some(y)) = (parts.first(), parts.get(1)) {
                if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                    coordinates.push(Position::new_xy(x, y));
                }
            }
        }

        let destination_position = destination.map(|destination| {
            let data: Vec<&str> = destination.split(',').collect();
            let x = data.first().and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            let y = data
                .get(1)
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0);
            let z = data
                .get(2)
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0);
            let rotation = data
                .get(3)
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0);

            Position::with_rotations(x, y, z, rotation, rotation)
        });

        WalkwaysEntrance::new(
            room_id,
            room_target_id,
            coordinates,
            destination_position,
        )
    }

    /// Mirrors `getDestination`.
    pub fn get_destination(&self, room: &Room, position: &Position) -> Option<WalkwaysEntrance> {
        if !room.is_public_room() {
            return None;
        }

        for entrance in &self.walkways {
            if entrance.get_room_id() != room.get_id() {
                continue;
            }

            let Some(model) = room.get_model() else {
                // Java `NullPointerException` equivalent.
                continue;
            };

            let mut destination = model.get_door_location();

            if let Some(entrance_destination) = entrance.get_destination() {
                destination = entrance_destination;
            }

            // The Java `Position.equals` compares `x` and `y` only.
            if destination.get_x() == position.get_x()
                && destination.get_y() == position.get_y()
            {
                return Some(entrance.clone());
            }
        }

        None
    }

    /// Mirrors `getWalkway`.
    pub fn get_walkway(&self, room: &Room, position: &Position) -> Option<WalkwaysEntrance> {
        if !room.is_public_room() {
            return None;
        }

        for entrance in &self.walkways {
            if entrance.get_room_id() != room.get_id() {
                continue;
            }

            for coord in entrance.get_from_coords() {
                // The Java `Position.equals` compares `x` and `y` only.
                if coord.get_x() == position.get_x() && coord.get_y() == position.get_y() {
                    return Some(entrance.clone());
                }
            }
        }

        None
    }

    /// Mirrors `getWalkways`.
    pub fn get_walkways(&self) -> Vec<WalkwaysEntrance> {
        self.walkways.clone()
    }
}
