//! Mirrors `net.h4bbo.lisbon.game.room.handlers.walkways.WalkwaysEntrance`.
use crate::game::pathfinder::position::Position;

#[derive(Clone)]
pub struct WalkwaysEntrance {
    room_id: i32,
    room_target_id: i32,
    from_coords: Vec<Position>,
    destination: Option<Position>,
}

impl WalkwaysEntrance {
    /// Mirrors the `WalkwaysEntrance(int, int, List<Position>, Position)`
    /// constructor.
    pub fn new(
        room_id: i32,
        room_target_id: i32,
        from_coords: Vec<Position>,
        destination: Option<Position>,
    ) -> Self {
        Self {
            room_id,
            room_target_id,
            from_coords,
            destination,
        }
    }

    /// Mirrors `getRoomId`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `getRoomTargetId`.
    pub fn get_room_target_id(&self) -> i32 {
        self.room_target_id
    }

    /// Mirrors `getFromCoords`.
    pub fn get_from_coords(&self) -> Vec<Position> {
        self.from_coords.clone()
    }

    /// Mirrors `getDestination`.
    pub fn get_destination(&self) -> Option<Position> {
        self.destination.clone()
    }
}
