//! Mirrors `net.h4bbo.lisbon.game.games.history.GameHistoryData`.

use std::collections::HashMap;

use crate::game::games::history::game_history_player::GameHistoryPlayer;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GameHistoryData {
    #[serde(rename = "teamCount", default)]
    team_count: i32,
    #[serde(default)]
    players: Vec<GameHistoryPlayer>,
}

impl GameHistoryData {
    /// Mirrors the `GameHistoryData` constructor.
    pub fn new() -> Self {
        Self {
            team_count: 0,
            players: Vec::new(),
        }
    }

    /// Mirrors `addPlayer(int, int, int)`.
    pub fn add_player(&mut self, user_id: i32, points: i32, team_id: i32) {
        self.players
            .push(GameHistoryPlayer::new(points, team_id, user_id));
    }

    /// Mirrors `getTeamCount()`.
    pub fn get_team_count(&self) -> i32 {
        self.team_count
    }

    /// Mirrors `setTeamCount(int)`.
    pub fn set_team_count(&mut self, team_count: i32) {
        self.team_count = team_count;
    }

    /// Mirrors `getTeamData()`.
    pub fn get_team_data(&self) -> HashMap<i32, Vec<GameHistoryPlayer>> {
        let mut team_history = HashMap::new();

        for team_id in 0..self.team_count {
            team_history.insert(
                team_id,
                self.players
                    .iter()
                    .filter(|player| player.get_team_id() == team_id)
                    .cloned()
                    .collect(),
            );
        }

        team_history
    }
}
