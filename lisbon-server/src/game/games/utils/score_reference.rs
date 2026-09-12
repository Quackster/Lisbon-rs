//! Mirrors `net.h4bbo.lisbon.game.games.utils.ScoreReference`.

use std::sync::Arc;

use crate::game::games::player::game_team::GameTeam;

pub struct ScoreReference {
    score: i32,
    game_team: Arc<GameTeam>,
    by: i32,
}

impl ScoreReference {
    /// Mirrors the `ScoreReference(int, GameTeam, int)` constructor.
    pub fn new(score: i32, game_team: Arc<GameTeam>, by: i32) -> Self {
        Self {
            score,
            game_team,
            by,
        }
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i32 {
        self.score
    }

    /// Mirrors `getGameTeam()`.
    pub fn get_game_team(&self) -> &GameTeam {
        &self.game_team
    }

    /// Mirrors `getBy()`.
    pub fn get_by(&self) -> i32 {
        self.by
    }
}
