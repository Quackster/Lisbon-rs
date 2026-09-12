//! Mirrors `net.h4bbo.lisbon.game.games.tasks.GameFinishTask`.
use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use rand::Rng;

use crate::dao::mysql::game_dao::GameDao;
use crate::game::game_scheduler::GameScheduler;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::game_manager::GameManager;
use crate::game::games::history::game_history::GameHistory;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::player::game_team::GameTeam;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::lisbon::Lisbon;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;

pub struct GameFinishTask<'a> {
    players: Vec<Arc<Mutex<GamePlayer>>>,
    sorted_team_list: Vec<Arc<GameTeam>>,
    game_type: GameType,
    game: &'a Game,
    game_history: &'a GameHistory,
}

impl<'a> GameFinishTask<'a> {
    /// Mirrors the `GameFinishTask(Game, GameHistory, GameType, List<GameTeam>, List<GamePlayer>)` constructor.
    pub fn new(
        game: &'a Game,
        game_history: &'a GameHistory,
        game_type: GameType,
        sorted_team_list: Vec<Arc<GameTeam>>,
        players: Vec<Arc<Mutex<GamePlayer>>>,
    ) -> Self {
        Self {
            players,
            sorted_team_list,
            game_type,
            game,
            game_history,
        }
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        let mut save_score = false;

        GameManager::get_instance().increment_finished_game_counter();

        if self.game.can_increase_points() {
            for game_player in self.game.get_active_players() {
                let mut game_player = game_player.lock();

                let score = game_player.get_score();
                let xp = if self.game_type == GameType::Battleball {
                    if score > 0 {
                        score / 294
                    } else {
                        0
                    }
                } else {
                    if score > 0 {
                        score / 3
                    } else {
                        0
                    }
                };
                game_player.set_xp(xp);
            }
        }

        if self.game.can_increase_points() {
            let team_count = self.sorted_team_list.len();

            // Port note: the Java `sortedTeamList.get(1)` index access is
            // unguarded (`ArrayIndexOutOfBoundsException` for a single
            // team); a guard is added here.
            let first_player_amount = self
                .sorted_team_list
                .first()
                .map_or(0, |team| team.get_players().len());
            let second_player_amount = self
                .sorted_team_list
                .get(1)
                .map_or(0, |team| team.get_players().len());
            let first_points = self
                .sorted_team_list
                .first()
                .map_or(0, |team| team.get_points());
            let second_points = self
                .sorted_team_list
                .get(1)
                .map_or(0, |team| team.get_points());

            if ((team_count == 1 && first_player_amount > 1)
                || (team_count > 1
                    && first_points > 0
                    && first_points != second_points))
                && first_player_amount > 0
                && (team_count > 1 && second_player_amount > 0)
            {
                save_score = true;

                let first_player = self.sorted_team_list[0]
                    .get_players()[0]
                    .lock()
                    .get_player()
                    .clone();
                let second_player = if team_count == 1 {
                    self.sorted_team_list[0]
                        .get_players()[1]
                        .lock()
                        .get_player()
                        .clone()
                } else {
                    self.sorted_team_list[1]
                        .get_players()[0]
                        .lock()
                        .get_player()
                        .clone()
                };

                let _first_ip =
                    NettyPlayerNetwork::get_ip_address(first_player.lock().get_network());
                let _second_ip =
                    NettyPlayerNetwork::get_ip_address(second_player.lock().get_network());
            }

            for game_player in &self.players {
                let mut set_map = HashMap::new();

                if save_score {
                    let score = game_player.lock().get_score() as i64;

                    let gp = game_player.lock();
                    let player = gp.get_player().lock();

                    if self.game_type == GameType::Battleball {
                        set_map.insert(
                            PlayerStatistic::BattleballMonthlyScores,
                            (player
                                .get_statistic_manager()
                                .get_long_value(PlayerStatistic::BattleballMonthlyScores)
                                + score)
                                .to_string(),
                        );
                        set_map.insert(
                            PlayerStatistic::BattleballPointsAllTime,
                            (player
                                .get_statistic_manager()
                                .get_long_value(PlayerStatistic::BattleballPointsAllTime)
                                + score)
                                .to_string(),
                        );
                    } else {
                        set_map.insert(
                            PlayerStatistic::SnowstormMonthlyScores,
                            (player
                                .get_statistic_manager()
                                .get_long_value(PlayerStatistic::SnowstormMonthlyScores)
                                + score)
                                .to_string(),
                        );
                        set_map.insert(
                            PlayerStatistic::SnowstormPointsAllTime,
                            (player
                                .get_statistic_manager()
                                .get_long_value(PlayerStatistic::SnowstormPointsAllTime)
                                + score)
                                .to_string(),
                        );
                    }

                    let xp = gp.get_xp() as i64;
                    set_map.insert(
                        PlayerStatistic::XpEarnedMonth,
                        (player
                            .get_statistic_manager()
                            .get_long_value(PlayerStatistic::XpEarnedMonth)
                            + xp)
                            .to_string(),
                    );
                    set_map.insert(
                        PlayerStatistic::XpAllTime,
                        (player
                            .get_statistic_manager()
                            .get_long_value(PlayerStatistic::XpAllTime)
                            + xp)
                            .to_string(),
                    );
                    player
                        .get_statistic_manager()
                        .set_values(gp.get_user_id(), &set_map);

                    if gp.get_xp() > 0 {
                        let is_winner = self
                            .sorted_team_list
                            .first()
                            .map_or(false, |team| {
                                team.get_players()
                                    .into_iter()
                                    .any(|candidate| Arc::ptr_eq(&candidate, game_player))
                            });

                        let credits_amount =
                            GameManager::get_instance().get_random_credits(is_winner);

                        if credits_amount > 0 {
                            GameScheduler::get_instance()
                                .queue_player_credits(gp.get_player().clone(), credits_amount);
                        }
                    }
                }
            }

            if save_score {
                let unique_id = Self::random_uuid();
                let game_history_data = self
                    .game_history
                    .get_game_history_data()
                    .map(|data| Lisbon::get_gson().to_json(data))
                    .unwrap_or_default();

                GameDao::save_team_history(
                    &unique_id,
                    self.game_history.get_name(),
                    self.game.get_game_creator_id(),
                    self.game_history.get_map_id(),
                    self.game_history.get_winning_team(),
                    self.game_history.get_winning_team_score(),
                    self.game_history.get_extra_data(),
                    self.game_type,
                    &game_history_data,
                );

                // Port note: the Java `refreshPlayedGames` call needs a
                // unique `&mut GameManager`; it is applied when the
                // singleton is uniquely held.
                let mut manager = GameManager::get_instance();
                if let Some(manager) = Arc::get_mut(&mut manager) {
                    manager.refresh_played_games();
                }
            }
        }
    }

    /// Mirrors `UUID.randomUUID().toString()` (the `java.util.UUID` is a
    /// `rand`-generated string).
    fn random_uuid() -> String {
        let mut rng = rand::thread_rng();
        let bytes: [u8; 16] = std::array::from_fn(|_| rng.gen());

        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15]
        )
    }
}
