//! Mirrors `net.h4bbo.lisbon.game.games.history.GameHistory`.

use crate::game::games::enums::game_type::GameType;
use crate::game::games::history::game_history_data::GameHistoryData;

#[derive(Clone, Debug)]
pub struct GameHistory {
    id: i32,
    name: String,
    game_creator: String,
    map_id: i32,
    winning_team: i32,
    winning_team_score: i32,
    extra_data: String,
    game_type: Option<GameType>,
    game_history_data: Option<GameHistoryData>,
}

impl GameHistory {
    /// Mirrors the `GameHistory()` constructor.
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            game_creator: String::new(),
            map_id: 0,
            winning_team: 0,
            winning_team_score: 0,
            extra_data: String::new(),
            game_type: None,
            game_history_data: None,
        }
    }

    /// Mirrors the `GameHistory(GameHistoryData)` constructor.
    pub fn with_data(game_history_data: GameHistoryData) -> Self {
        let mut instance = Self::new();
        instance.game_history_data = Some(game_history_data);
        instance
    }

    /// Mirrors `getHistoryData()`.
    pub fn get_history_data(&self) -> Option<&GameHistoryData> {
        self.game_history_data.as_ref()
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `setId(int)`.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `setName(String)`.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Mirrors `getGameCreator()`.
    pub fn get_game_creator(&self) -> &str {
        &self.game_creator
    }

    /// Mirrors `setGameCreator(String)`.
    pub fn set_game_creator(&mut self, game_creator: &str) {
        self.game_creator = game_creator.to_string();
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `setMapId(int)`.
    pub fn set_map_id(&mut self, map_id: i32) {
        self.map_id = map_id;
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> Option<GameType> {
        self.game_type
    }

    /// Mirrors `setGameType(GameType)`.
    pub fn set_game_type(&mut self, game_type: Option<GameType>) {
        self.game_type = game_type;
    }

    /// Mirrors `getGameHistoryData()`.
    pub fn get_game_history_data(&self) -> Option<&GameHistoryData> {
        self.game_history_data.as_ref()
    }

    /// Mirrors `setGameHistoryData(GameHistoryData)`.
    pub fn set_game_history_data(&mut self, game_history_data: Option<GameHistoryData>) {
        self.game_history_data = game_history_data;
    }

    /// Mirrors `getWinningTeam()`.
    pub fn get_winning_team(&self) -> i32 {
        self.winning_team
    }

    /// Mirrors `setWinningTeam(int)`.
    pub fn set_winning_team(&mut self, winning_team: i32) {
        self.winning_team = winning_team;
    }

    /// Mirrors `getWinningTeamScore()`.
    pub fn get_winning_team_score(&self) -> i32 {
        self.winning_team_score
    }

    /// Mirrors `setWinningTeamScore(int)`.
    pub fn set_winning_team_score(&mut self, winning_team_score: i32) {
        self.winning_team_score = winning_team_score;
    }

    /// Mirrors `getExtraData()`.
    pub fn get_extra_data(&self) -> &str {
        &self.extra_data
    }

    /// Mirrors `setExtraData(String)`.
    pub fn set_extra_data(&mut self, extra_data: &str) {
        self.extra_data = extra_data.to_string();
    }

    /// Mirrors `getAllowedPowerUps()` (unparsable entries are skipped
    /// instead of throwing).
    pub fn get_allowed_power_ups(&self) -> Vec<i32> {
        if !self.extra_data.is_empty() {
            return self
                .extra_data
                .split(',')
                .filter_map(|part| part.parse::<i32>().ok())
                .collect();
        }

        Vec::new()
    }

    /// Mirrors `getGameLength()` (`TimeUnit.MINUTES.toSeconds(...)` is the
    /// literal; a parse failure yields 0 instead of throwing).
    pub fn get_game_length(&self) -> i32 {
        if self.game_type == Some(GameType::Snowstorm) {
            if let Ok(game_length_choice) = self.extra_data.parse::<i32>() {
                match game_length_choice {
                    1 => return 120,
                    2 => return 180,
                    3 => return 300,
                    _ => {}
                }
            }
        }

        0
    }
}
