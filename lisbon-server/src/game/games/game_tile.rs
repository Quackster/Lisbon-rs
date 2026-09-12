//! Mirrors `net.h4bbo.lisbon.game.games.GameTile`.
// Port note: the Java class is abstract but declares no abstract methods.

use crate::game::pathfinder::position::Position;

#[derive(Clone, Debug)]
pub struct GameTile {
    position: Position,
    is_spawn_occupied: bool,
}

impl GameTile {
    /// Mirrors the `GameTile(Position)` constructor.
    pub fn new(position: Position) -> Self {
        Self {
            position,
            is_spawn_occupied: false,
        }
    }

    /// Get the position of this tile.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Get whether this tile has been used as a spawn point.
    pub fn is_spawn_occupied(&self) -> bool {
        self.is_spawn_occupied
    }

    /// Set whether this tile has been used as a spawn point.
    pub fn set_spawn_occupied(&mut self, spawn_occupied: bool) {
        self.is_spawn_occupied = spawn_occupied;
    }
}
