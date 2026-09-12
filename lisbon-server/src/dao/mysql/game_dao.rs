//! Mirrors `net.h4bbo.lisbon.dao.mysql.GameDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::games::battleball::battle_ball_map::BattleBallMap;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_spawn::GameSpawn;
use crate::game::games::history::game_history::GameHistory;
use crate::game::games::history::game_history_data::GameHistoryData;
use crate::game::games::player::game_rank::GameRank;
use crate::game::room::models::room_model::RoomModel;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

/// The `game_type` column is stored with the Java `name()` value (upper-case);
/// `GameType::get_name` is lower-case, so a dedicated mapping is used.
fn java_game_type_name(game_type: GameType) -> &'static str {
    match game_type {
        GameType::Battleball => "BATTLEBALL",
        GameType::Snowstorm => "SNOWSTORM",
        GameType::WobbleSquabble => "WOBBLE_SQUABBLE",
    }
}

pub struct GameDao;

impl GameDao {
    /// Mirrors `getRanks()`.
    pub fn get_ranks() -> Vec<GameRank> {
        let mut ranks = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM games_ranks") {
            if let (Some(id), Some(game_type), Some(title), Some(min_points), Some(max_points)) = (
                row.i32("id"),
                row.str("type"),
                row.str("title"),
                row.i32("min_points"),
                row.i32("max_points"),
            ) {
                ranks.push(GameRank::new(
                    id,
                    &game_type,
                    &title,
                    min_points,
                    max_points,
                ));
            }
        }

        ranks
    }

    /// Mirrors `getGameMaps()`.
    pub fn get_game_maps() -> Vec<RoomModel> {
        let mut maps = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM games_maps") {
            if let (Some(map_id), Some(game_type), Some(heightmap)) = (
                row.i32("map_id"),
                row.str("game_type"),
                row.str("heightmap"),
            ) {
                let model_name = if game_type != "battleball" {
                    format!("ss_arena_{map_id}")
                } else {
                    format!("bb_arena_{map_id}")
                };

                maps.push(RoomModel::new(
                    &model_name,
                    &model_name,
                    i32::MAX,
                    i32::MAX,
                    f64::MAX,
                    0,
                    &heightmap,
                    None,
                ));
            }
        }

        maps
    }

    /// Mirrors `getGameSpawns()`.
    pub fn get_game_spawns() -> Vec<GameSpawn> {
        let mut spawns = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM games_player_spawns") {
            if let (Some(team_id), Some(map_id), Some(game_type), Some(x), Some(y), Some(rotation)) = (
                row.i32("team_id"),
                row.i32("map_id"),
                row.str("type"),
                row.i32("x"),
                row.i32("y"),
                row.i32("rotation"),
            ) {
                spawns.push(GameSpawn::new(
                    team_id,
                    map_id,
                    &game_type,
                    x,
                    y,
                    rotation,
                ));
            }
        }

        spawns
    }

    /// Mirrors `getBattleballTileMaps()`.
    pub fn get_battleball_tile_maps() -> Vec<BattleBallMap> {
        let mut maps = Vec::new();

        for row in Storage::get_storage()
            .query_all("SELECT * FROM games_maps WHERE game_type = 'battleball'")
        {
            if let (Some(map_id), Some(tile_map)) = (row.i32("map_id"), row.str("tile_map")) {
                maps.push(BattleBallMap::new(map_id, GameType::Battleball, &tile_map));
            }
        }

        maps
    }

    /// Mirrors `resetMonthlyXp()`.
    pub fn reset_monthly_xp() {
        Storage::get_storage().execute(
            "UPDATE users_statistics SET battleball_score_month = 0, snowstorm_score_month = 0, wobble_squabble_score_month = 0, xp_earned_month = 0 WHERE (battleball_score_month > 0) OR (snowstorm_score_month > 0) OR (wobble_squabble_score_month > 0) OR (xp_earned_month > 0)",
        );
    }

    /// Mirrors `saveTeamHistory(String, String, int, int, int, int, String,
    /// GameType, String)`.
    pub fn save_team_history(
        unique_id: &str,
        game_name: &str,
        map_creator: i32,
        map_id: i32,
        winning_team: i32,
        winning_team_score: i32,
        extra_data: &str,
        game_type: GameType,
        game_history_data: &str,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO games_played_history (id, game_name, game_creator, game_type, map_id, winning_team, winning_team_score, extra_data, team_data) VALUES ('{}', '{}', {map_creator}, '{}', {map_id}, {winning_team}, {winning_team_score}, '{}', '{}')",
            escape(unique_id),
            escape(game_name),
            java_game_type_name(game_type),
            escape(extra_data),
            escape(game_history_data)
        ));
    }

    /// Mirrors `getLastPlayedGames(GameType)`.
    pub fn get_last_played_games(game_type: GameType) -> Vec<GameHistory> {
        let mut games = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT games_played_history.*, users.username AS game_creator_name FROM games_played_history INNER JOIN users ON users.id = games_played_history.game_creator WHERE game_type = '{}' ORDER BY played_at DESC LIMIT 15",
                java_game_type_name(game_type)
            ),
        ) {
            let mut game_history = GameHistory::new();

            if let Some(game_name) = row.str("game_name") {
                game_history.set_name(&game_name);
            }
            if let Some(game_creator_name) = row.str("game_creator_name") {
                game_history.set_game_creator(&game_creator_name);
            }
            if let Some(map_id) = row.i32("map_id") {
                game_history.set_map_id(map_id);
            }
            if let Some(row_game_type) = row.str("game_type") {
                game_history.set_game_type(GameType::from_str(&row_game_type));
            }
            if let Some(winning_team) = row.i32("winning_team") {
                game_history.set_winning_team(winning_team);
            }
            if let Some(winning_team_score) = row.i32("winning_team_score") {
                game_history.set_winning_team_score(winning_team_score);
            }
            if let Some(extra_data) = row.str("extra_data") {
                game_history.set_extra_data(&extra_data);
            }
            if let Some(team_data) = row.str("team_data") {
                if let Some(data) = crate::lisbon::Lisbon::get_gson().from_json::<GameHistoryData>(&team_data) {
                    game_history.set_game_history_data(Some(data));
                }
            }

            games.push(game_history);
        }

        games
    }
}
