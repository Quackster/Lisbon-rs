//! Mirrors `net.h4bbo.lisbon.game.games.utils.FinishedGame`.
use std::collections::HashMap;
use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_team::GameTeam;
use crate::util::date_util::DateUtil;

pub struct FinishedGame {
    id: i32,
    map_id: i32,
    name: String,
    map_creator: String,
    game_type: GameType,
    power_ups: Option<Vec<i32>>,
    team_scores: HashMap<i32, FinishedGameTeam>,
    expire_time: i64,
}

impl FinishedGame {
    /// Mirrors the `FinishedGame(Game)` constructor.
    pub fn new(game: &Game) -> Self {
        let id = game.get_id();
        let map_id = game.get_map_id();
        let name = game.get_name();
        let map_creator = game.get_game_creator();
        let game_type = game.get_game_type();

        let power_ups = if game_type == GameType::Battleball {
            game
                .as_battle_ball()
                .map(|battleball_game| battleball_game.get_allowed_power_ups())
        } else {
            None
        };

        let expire_time = DateUtil::get_current_time_seconds() as i64
            + GameManager::get_instance().get_listing_expiry_time() as i64;

        let mut team_scores = HashMap::new();

        for i in 0..game.get_team_amount() {
            team_scores.insert(
                i,
                FinishedGameTeam::new(game.get_teams().get(i as usize).cloned()),
            );
        }

        Self {
            id,
            map_id,
            name,
            map_creator,
            game_type,
            power_ups,
            team_scores,
            expire_time,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `getMapCreator()`.
    pub fn get_map_creator(&self) -> &str {
        &self.map_creator
    }

    /// Mirrors `getTeamScores()`.
    pub fn get_team_scores(&self) -> &HashMap<i32, FinishedGameTeam> {
        &self.team_scores
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getAllowedPowerUps()`.
    pub fn get_allowed_power_ups(&self) -> Option<&Vec<i32>> {
        self.power_ups.as_ref()
    }

    /// Mirrors `getExpireTime()`.
    pub fn get_expire_time(&self) -> i64 {
        self.expire_time
    }
}

pub struct FinishedGameTeam {
    score: i32,
    player_scores: Vec<(String, i32)>,
}

impl FinishedGameTeam {
    /// Mirrors the `FinishedGameTeam(GameTeam)` constructor (the
    /// `commons-lang3 Pair<String, Integer>` is a tuple).
    pub fn new(game_team: Option<Arc<Mutex<GameTeam>>>) -> Self {
        let score = game_team
            .as_ref()
            .map(|game_team| game_team.lock().get_points())
            .unwrap_or(0);
        let mut player_scores = Vec::new();

        if let Some(game_team) = game_team {
            for game_player in game_team.lock().get_players() {
                let game_player = game_player.lock();
                let name = game_player
                    .get_player()
                    .lock()
                    .get_details()
                    .get_name()
                    .to_string();
                player_scores.push((name, game_player.get_score()));
            }
        }

        Self {
            score,
            player_scores,
        }
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i32 {
        self.score
    }

    /// Mirrors `getPlayerScores()`.
    pub fn get_player_scores(&self) -> &[(String, i32)] {
        &self.player_scores
    }
}
