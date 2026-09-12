//! Mirrors `net.h4bbo.lisbon.game.games.player.GameRank`.

use crate::game::games::enums::game_type::GameType;

#[derive(Clone, Debug)]
pub struct GameRank {
    id: i32,
    type_: GameType,
    title: String,
    min_points: i32,
    max_points: i32,
}

impl GameRank {
    /// Mirrors the `GameRank(int, String, String, int, int)` constructor
    /// (the unknown-name `valueOf` throw is an `expect` here).
    pub fn new(id: i32, type_: &str, title: &str, min_points: i32, max_points: i32) -> Self {
        Self {
            id,
            type_: GameType::from_str(&type_.to_uppercase()).expect("unknown GameType"),
            title: title.to_string(),
            min_points,
            max_points,
        }
    }

    /// Get the game rank ID.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Get the game type for this rank.
    pub fn get_type(&self) -> GameType {
        self.type_
    }

    /// Get the rank title.
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Get the minimum amount of points required for this game.
    pub fn get_min_points(&self) -> i32 {
        self.min_points
    }

    /// Get the maximum amount of points required for this game.
    pub fn get_max_points(&self) -> i32 {
        self.max_points
    }
}
