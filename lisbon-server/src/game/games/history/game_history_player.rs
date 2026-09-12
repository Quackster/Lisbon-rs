//! Mirrors `net.h4bbo.lisbon.game.games.history.GameHistoryPlayer`.

use crate::dao::mysql::player_dao::PlayerDao;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GameHistoryPlayer {
    #[serde(default)]
    score: i32,
    #[serde(rename = "teamId", default)]
    team_id: i32,
    #[serde(rename = "userId", default)]
    user_id: i32,
    #[serde(default)]
    username: Option<String>,
}

impl GameHistoryPlayer {
    /// Mirrors the `GameHistoryPlayer(int, int, int)` constructor.
    pub fn new(score: i32, team_id: i32, user_id: i32) -> Self {
        Self {
            score,
            team_id,
            user_id,
            username: None,
        }
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i32 {
        self.score
    }

    /// Mirrors `getTeamId()`.
    pub fn get_team_id(&self) -> i32 {
        self.team_id
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getName()` (the lazy `PlayerDao` lookup is retained).
    pub fn get_name(&mut self) -> Option<&str> {
        if self.username.is_none() {
            self.username = PlayerDao::get_name(self.user_id);
        }

        self.username.as_deref()
    }
}
