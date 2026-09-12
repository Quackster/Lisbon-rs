//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormSpawn`.
#[derive(Clone, Debug)]
pub struct SnowStormSpawn {
    position: crate::game::pathfinder::position::Position,
    radius: i32,
    min_distance: i32,
}

impl SnowStormSpawn {
    /// Mirrors the `SnowStormSpawn(int, int, int, int)` constructor.
    pub fn new(x: i32, y: i32, radius: i32, min_distance: i32) -> Self {
        Self {
            position: crate::game::pathfinder::position::Position::new_xy(x, y),
            radius,
            min_distance,
        }
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> &crate::game::pathfinder::position::Position {
        &self.position
    }

    /// Mirrors `getRadius()`.
    pub fn get_radius(&self) -> i32 {
        self.radius
    }

    /// Mirrors `getMinDistance()`.
    pub fn get_min_distance(&self) -> i32 {
        self.min_distance
    }
}
