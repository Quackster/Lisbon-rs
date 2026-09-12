//! Mirrors `net.h4bbo.lisbon.game.games.history.ScoreEntry`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct ScoreEntry {
    player_name: String,
    score: i64,
    position: i32,
}

impl ScoreEntry {
    /// Mirrors the `ScoreEntry(String, long, int)` constructor.
    pub fn new(player_name: &str, score: i64, position: i32) -> Self {
        Self {
            player_name: player_name.to_string(),
            score,
            position,
        }
    }

    /// Mirrors `getPlayerName()`.
    pub fn get_player_name(&self) -> &str {
        &self.player_name
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i64 {
        self.score
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> i32 {
        self.position
    }
}
