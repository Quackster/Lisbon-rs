//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.mapping.SnowStormItem`.
use crate::game::pathfinder::position::Position;

#[derive(Clone, Debug)]
pub struct SnowStormItem {
    item_id: String,
    item_name: String,
    x: i32,
    y: i32,
    z: i32,
    rotation: i32,
    height: i32,
}

impl SnowStormItem {
    /// Mirrors the `SnowStormItem(String, String, int, int, int, int, int)` constructor.
    pub fn new(
        item_id: String,
        item_name: String,
        x: i32,
        y: i32,
        z: i32,
        rotation: i32,
        height: i32,
    ) -> Self {
        Self {
            item_id,
            item_name,
            x,
            y,
            z,
            rotation,
            height,
        }
    }

    /// Mirrors `isSnowballMachine()`.
    pub fn is_snowball_machine(&self) -> bool {
        self.item_name.eq_ignore_ascii_case("snowball_machine")
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> &str {
        &self.item_id
    }

    /// Mirrors `getItemName()`.
    pub fn get_item_name(&self) -> &str {
        &self.item_name
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `getZ()`.
    pub fn get_z(&self) -> i32 {
        self.z
    }

    /// Mirrors `getRotation()`.
    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y, self.z as f64)
    }

    /// Mirrors `getHeight()`.
    pub fn get_height(&self) -> i32 {
        self.height
    }
}
