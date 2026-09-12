//! Mirrors `net.h4bbo.lisbon.game.games.GameManager`.
//!
//! The Java `expiryLoop` (`ScheduledFuture`) has no tokio equivalent
//! yet; it was never started in Java either.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use rand::Rng;

use crate::dao::mysql::game_dao::GameDao;
use crate::game::games::battleball::battle_ball_map::BattleBallMap;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::game_spawn::GameSpawn;
use crate::game::games::history::game_history::GameHistory;
use crate::game::games::player::game_rank::GameRank;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::game::room::models::room_model::RoomModel;
use crate::log::Log;
use crate::util::config::game_configuration::GameConfiguration;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<GameManager>>> = RwLock::new(None);
}

/// Mirrors `GameManager`.
pub struct GameManager {
    game_counter: AtomicI32,
    finished_game_counter: AtomicI32,
    spawn_list: Vec<GameSpawn>,
    rank_list: Vec<GameRank>,
    model_list: Vec<RoomModel>,
    battleball_tile_maps: Vec<BattleBallMap>,
    games: Mutex<Vec<Game>>,
    last_played_games: HashMap<GameType, Vec<GameHistory>>,
}

impl GameManager {
    /// Mirrors the `GameManager` constructor.
    fn new() -> Self {
        let mut instance = Self {
            game_counter: AtomicI32::new(0),
            finished_game_counter: AtomicI32::new(0),
            spawn_list: GameDao::get_game_spawns(),
            rank_list: GameDao::get_ranks(),
            model_list: GameDao::get_game_maps(),
            battleball_tile_maps: GameDao::get_battleball_tile_maps(),
            games: Mutex::new(Vec::new()),
            last_played_games: HashMap::new(),
        };

        instance.refresh_played_games();

        instance
    }

    /// Mirrors `refreshPlayedGames()` (the `GameDao.getLastPlayedGames()`
    /// bulk variant is not ported; per-type loads are used instead).
    pub fn refresh_played_games(&mut self) {
        self.last_played_games.clear();

        for game_type in GameType::values() {
            let mut list = GameDao::get_last_played_games(*game_type);

            for game in list.iter_mut() {
                game.set_id(self.game_counter.fetch_add(1, Ordering::SeqCst));
            }

            self.last_played_games.insert(*game_type, list);
        }
    }

    /// Mirrors `getRandomCredits(boolean)`.
    pub fn get_random_credits(&self, is_winner: bool) -> i32 {
        let mut max_credits = 0;
        let mut min_credits = 0;

        let range = GameConfiguration::get_instance().get_string(
            &format!("reward.credits.{}.range", if is_winner { "winner" } else { "loser" }),
        );
        let range_data: Vec<&str> = range.split('-').collect();

        match (
            range_data.first().and_then(|part| part.parse::<i32>().ok()),
            range_data.get(1).and_then(|part| part.parse::<i32>().ok()),
        ) {
            (Some(min), Some(max)) => {
                min_credits = min;
                max_credits = max;
            }
            _ => {
                Log::get_error_logger()
                    .error("Error when handling give random credits: invalid range");
            }
        }

        if min_credits == max_credits {
            return max_credits;
        }

        rand::thread_rng().gen_range(min_credits..=max_credits)
    }

    /// Get the game spawn list by game type, map id and team id.
    pub fn get_game_spawns(
        &self,
        game_type: GameType,
        map_id: i32,
        team_id: i32,
    ) -> Vec<GameSpawn> {
        self.spawn_list
            .iter()
            .filter(
                |game_spawn| {
                    game_spawn.get_game_type() == game_type
                        && game_spawn.get_map_id() == map_id
                        && game_spawn.get_team_id() == team_id
                },
            )
            .cloned()
            .collect()
    }

    /// Get the game spawn by game type, map id and team id.
    pub fn get_battleball_tile_map(&self, map_id: i32) -> Option<&BattleBallMap> {
        self.battleball_tile_maps
            .iter()
            .find(|tile_map| tile_map.get_map_id() == map_id)
    }

    /// Gets a game instance by specified game id.
    pub fn get_game_by_id(&self, game_id: i32) -> Option<Game> {
        self.games
            .lock()
            .iter()
            .find(|game| game.get_id() == game_id)
            .cloned()
    }

    /// Mirrors `getGames().add(Game)`.
    pub fn add_game(&self, game: Game) {
        self.games.lock().push(game)
    }

    /// Mirrors `getGames().remove(Game)`.
    pub fn remove_game(&self, game_id: i32) {
        self.games
            .lock()
            .retain(|game| game.get_id() != game_id)
    }

    /// Get the list of games by type.
    pub fn get_games_by_type(&self, game_type: GameType) -> Vec<Game> {
        self.games
            .lock()
            .iter()
            .filter(|game| game.get_game_type() == game_type)
            .cloned()
            .collect()
    }

    /// Get the list of started games by type.
    pub fn get_started_games_by_type(&self, game_type: GameType) -> Vec<Game> {
        self.games
            .lock()
            .iter()
            .filter(
                |game| {
                    game.get_game_type() == game_type
                        && game.get_game_state() == GameState::Started
                },
            )
            .cloned()
            .collect()
    }

    /// Get the list of game ranks by type.
    pub fn get_ranks_by_type(&self, game_type: GameType) -> Vec<GameRank> {
        self.rank_list
            .iter()
            .filter(|rank| rank.get_type() == game_type)
            .cloned()
            .collect()
    }

    /// Get the instance of `GameManager`.
    pub fn get_instance() -> Arc<GameManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }

        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Reload the instance of `GameManager` (the game counter is retained,
    /// mirroring the Java `reset`).
    pub fn reset() {
        let game_counter = INSTANCE
            .read()
            .as_ref()
            .map(|instance| instance.get_game_counter())
            .unwrap_or(0);

        INSTANCE.write().take();

        Self::get_instance()
            .game_counter
            .store(game_counter, Ordering::SeqCst);
    }

    /// Creates a new game id for the game.
    pub fn create_id(&self) -> i32 {
        self.game_counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Gets the game id counter (the Java `AtomicInteger` is an `i32`
    /// snapshot here).
    pub fn get_game_counter(&self) -> i32 {
        self.game_counter.load(Ordering::SeqCst)
    }

    /// Gets the list of currently active games.
    pub fn get_games(&self) -> Vec<Game> {
        self.games.lock().clone()
    }

    /// Gets the restart time for the specified game type.
    pub fn get_restart_seconds(&self, game_type: GameType) -> i32 {
        GameConfiguration::get_instance().get_integer(&format!(
            "{}.restart.game.seconds",
            game_type.get_name()
        ))
    }

    /// Gets the game time for the specified game type.
    pub fn get_lifetime_seconds(&self, game_type: GameType) -> i32 {
        GameConfiguration::get_instance().get_integer(&format!(
            "{}.game.lifetime.seconds",
            game_type.get_name()
        ))
    }

    /// Gets the game time for the specified game type.
    pub fn get_preparing_seconds(&self, game_type: GameType) -> i32 {
        GameConfiguration::get_instance().get_integer(&format!(
            "{}.preparing.game.seconds",
            game_type.get_name()
        ))
    }

    /// Get the amount of seconds allowed for a finished game to persist on
    /// the instance list before it's removed.
    pub fn get_listing_expiry_time(&self) -> i32 {
        GameConfiguration::get_instance().get_integer("game.finished.listing.expiry.seconds")
    }

    /// Get model by type and map id.
    pub fn get_model(&self, r#type: GameType, map_id: i32) -> Option<&RoomModel> {
        let prefix = if r#type == GameType::Battleball { "bb" } else { "ss" };

        self.model_list
            .iter()
            .find(|room_model| room_model.get_name() == &format!("{}_arena_{}", prefix, map_id))
    }

    /// Mirrors `getMaps()`.
    pub fn get_maps(&self) -> &[RoomModel] {
        &self.model_list
    }

    /// Gets the finished game counter (the Java `AtomicInteger` is an
    /// `i32` snapshot here).
    pub fn get_finished_game_counter(&self) -> i32 {
        self.finished_game_counter.load(Ordering::SeqCst)
    }

    /// Mirrors `getFinishedGameCounter().incrementAndGet()`.
    pub fn increment_finished_game_counter(&self) -> i32 {
        self.finished_game_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Mirrors `getLastPlayedGames(GameType)`.
    pub fn get_last_played_games(
        &self,
        game_type: GameType,
    ) -> Option<&Vec<GameHistory>> {
        self.last_played_games.get(&game_type)
    }

    /// Get the game rank by the player points.
    pub fn get_rank_by_points(
        &self,
        r#type: GameType,
        player: &Player,
    ) -> Option<&GameRank> {
        let score = match r#type {
            GameType::Battleball => player
                .get_statistic_manager()
                .get_int_value(PlayerStatistic::BattleballPointsAllTime),
            GameType::Snowstorm => player
                .get_statistic_manager()
                .get_int_value(PlayerStatistic::SnowstormPointsAllTime),
            _ => 0,
        };

        for rank in &self.rank_list {
            if score >= rank.get_min_points() {
                if rank.get_max_points() == 0 || score <= rank.get_max_points() {
                    return Some(rank);
                }
            }
        }

        None
    }

    /// Mirrors `getFinishedGameById(GameType, int)`.
    pub fn get_finished_game_by_id(
        &self,
        game_type: GameType,
        game_id: i32,
    ) -> Option<&GameHistory> {
        self.last_played_games
            .get(&game_type)?
            .iter()
            .find(|game| game.get_id() == game_id)
    }
}
