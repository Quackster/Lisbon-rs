//! Mirrors `net.h4bbo.lisbon.game.games.wobblesquabble.WobbleSquabbleGame`.
//! The scheduled 1500ms end-of-game cleanup (`GameScheduler
//! .getService().schedule(...)`) runs inline (the game is only reachable
//! through the task manager's `Mutex`). The Java `try/catch` (exception
//! logging) has no Rust equivalent.
use std::sync::Arc;

use parking_lot::Mutex;
use rand::Rng;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::entity::entity::Entity;
use crate::game::games::wobblesquabble::wobble_squabble_manager::WobbleSquabbleManager;
use crate::game::games::wobblesquabble::wobble_squabble_move::WobbleSquabbleMove;
use crate::game::games::wobblesquabble::wobble_squabble_player::WobbleSquabblePlayer;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::room::Room;
use crate::messages::outgoing::user::currencies::ticket_balance::TICKET_BALANCE;
use crate::messages::outgoing::wobblesquabble::pt_bothlose::PT_BOTHLOSE;
use crate::messages::outgoing::wobblesquabble::pt_end::PT_END;
use crate::messages::outgoing::wobblesquabble::pt_start::PT_START;
use crate::messages::outgoing::wobblesquabble::pt_status::PT_STATUS;
use crate::messages::outgoing::wobblesquabble::pt_win::PT_WIN;
use crate::messages::types::MessageComposer;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;
use crate::util::date_util::DateUtil;

pub struct WobbleSquabbleGame {
    room: Room,
    first_player: Arc<Mutex<WobbleSquabblePlayer>>,
    second_player: Arc<Mutex<WobbleSquabblePlayer>>,
    has_game_started: bool,
    has_game_ended: bool,
    game_started: i32,
}

impl WobbleSquabbleGame {
    /// Mirrors the `WobbleSquabbleGame(Player, Player)` constructor (the
    /// `Arc` cycle is broken with `Arc::new_cyclic` + `Weak`).
    pub fn new(
        first_player: Arc<Mutex<Player>>,
        second_player: Arc<Mutex<Player>>,
    ) -> Self {
        Arc::into_inner(Arc::new_cyclic(|weak_self| {
            let room = first_player
                .lock()
                .get_room_user()
                .and_then(|room_user| room_user.get_room())
                .unwrap_or_default();

            let left_seat = first_player
                .lock()
                .get_room_user()
                .map(|room_user| room_user.get_position())
                .filter(|position| {
                    position.get_x() == Self::LEFT_X && position.get_y() == Self::LEFT_Y
                })
                .is_some();

            let mut first = WobbleSquabblePlayer::new(weak_self, first_player, -1);
            let mut second = WobbleSquabblePlayer::new(weak_self, second_player, -1);

            if left_seat {
                first.set_order(0);
                second.set_order(1);
            } else {
                second.set_order(0);
                first.set_order(1);
            }

            Self {
                room,
                first_player: Arc::new(Mutex::new(first)),
                second_player: Arc::new(Mutex::new(second)),
                has_game_started: false,
                has_game_ended: false,
                game_started: 0,
            }
        }))
        .expect("single strong reference")
    }

    /// Mirrors the `LEFT_X` constant.
    pub const LEFT_X: i32 = 21;

    /// Mirrors the `LEFT_Y` constant.
    pub const LEFT_Y: i32 = 15;

    /// Mirrors `send(MessageComposer)` (the per-player sends are commented
    /// out in the Java source).
    pub fn send(&self, composer: &dyn MessageComposer) {
        self.room.send(composer)
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        if self.has_game_ended {
            return;
        }

        let Some(ws_player1) = self.get_player_arc(0) else {
            return;
        };
        let Some(ws_player2) = self.get_player_arc(1) else {
            return;
        };

        if !self.has_game_started {
            self.has_game_started = true;
            self.game_started = DateUtil::get_current_time_seconds();
            self.send(&PT_START::new(
                Arc::clone(&ws_player1),
                Arc::clone(&ws_player2),
            ));

            // Set positions
            ws_player1.lock().set_position(-3);
            ws_player2.lock().set_position(4);
            return;
        }

        // If either requires a status update, send the update
        if ws_player1.lock().is_requires_update() || ws_player2.lock().is_requires_update() {
            self.update_player(0);
            self.update_player(1);

            self.send(&PT_STATUS::new(
                &ws_player1.lock(),
                &ws_player2.lock(),
            ));

            ws_player1.lock().reset_actions();
            ws_player2.lock().reset_actions();
        }

        // If the game has timed out, too bad!
        if DateUtil::get_current_time_seconds() as i64
            > self.game_started as i64 + WobbleSquabbleManager::WS_GAME_TIMEOUT_SECS as i64
        {
            self.send(&PT_BOTHLOSE);
            self.end_game(-1); // Tied!
            return;
        }

        let mut loser = -1;
        let mut winner = -1;

        // If one of us gets a bit tipsy and off-balance, end the game!
        if !ws_player1.lock().is_balancing() {
            loser = ws_player1.lock().get_order();
            winner = ws_player2.lock().get_order();
        }

        if !ws_player2.lock().is_balancing() {
            loser = ws_player2.lock().get_order();
            winner = ws_player1.lock().get_order();
        }

        // If we have a loser and a winner, end the game!
        if loser != -1 && winner != -1 {
            self.end_game(winner);
        }
    }

    /// Mirrors `updatePlayer(int)`.
    fn update_player(&mut self, ws_player_order: i32) {
        let Some(ws_player) = self.get_player_arc(ws_player_order) else {
            return;
        };
        let Some(ws_opponent) = self.get_player_arc(
            if ws_player.lock().get_order() == 1 {
                0
            } else {
                1
            },
        ) else {
            return;
        };

        let opponent_distance = (ws_player.lock().get_position()
            - ws_opponent.lock().get_position())
        .abs();

        let move_type = ws_player.lock().get_move();
        match move_type {
            WobbleSquabbleMove::BalanceLeft => {
                let balance_calculated =
                    WobbleSquabbleManager::WS_BALANCE_POINTS + rand::thread_rng().gen_range(0..10);
                ws_player
                    .lock()
                    .set_balance(ws_player.lock().get_balance() - balance_calculated);
            }

            WobbleSquabbleMove::BalanceRight => {
                let balance_calculated =
                    WobbleSquabbleManager::WS_BALANCE_POINTS + rand::thread_rng().gen_range(0..10);
                ws_player
                    .lock()
                    .set_balance(ws_player.lock().get_balance() + balance_calculated);
            }

            WobbleSquabbleMove::HitLeft => {
                // Are we standing next to our opponent?
                // (wsPlayer.getPosition() + 1) == wsOpponent.getPosition() ||
                // (wsPlayer.getPosition() - 1) == wsOpponent.getPosition() is
                // commented out in the Java source.
                if opponent_distance <= 2 {
                    ws_opponent.lock().set_hit(true);

                    let balance_calculated =
                        WobbleSquabbleManager::WS_HIT_POINTS + rand::thread_rng().gen_range(0..10);
                    ws_opponent
                        .lock()
                        .set_balance(ws_opponent.lock().get_balance() + balance_calculated);
                } else {
                    let balance_calculated = WobbleSquabbleManager::WS_HIT_BALANCE_POINTS
                        + rand::thread_rng().gen_range(0..10);
                    ws_player
                        .lock()
                        .set_balance(ws_player.lock().get_balance() + balance_calculated);
                }
            }

            WobbleSquabbleMove::HitRight => {
                // Are we standing next to our opponent?
                // (wsPlayer.getPosition() + 1) == wsOpponent.getPosition() ||
                // (wsPlayer.getPosition() - 1) == wsOpponent.getPosition() is
                // commented out in the Java source.
                if opponent_distance <= 2 {
                    ws_opponent.lock().set_hit(true);

                    let balance_calculated =
                        WobbleSquabbleManager::WS_HIT_POINTS + rand::thread_rng().gen_range(0..10);
                    ws_opponent
                        .lock()
                        .set_balance(ws_opponent.lock().get_balance() - balance_calculated);
                } else {
                    let balance_calculated = WobbleSquabbleManager::WS_HIT_BALANCE_POINTS
                        + rand::thread_rng().gen_range(0..10);
                    ws_player
                        .lock()
                        .set_balance(ws_player.lock().get_balance() - balance_calculated);
                }
            }

            WobbleSquabbleMove::WalkForward => {
                // Calculate new position
                let new_position = ws_player.lock().get_position() - 1;

                if new_position >= -3 && new_position <= 4 {
                    if new_position != ws_opponent.lock().get_position() {
                        ws_player.lock().set_position(new_position);
                    }
                }
            }

            WobbleSquabbleMove::WalkBackward => {
                // Calculate new position
                let new_position = ws_player.lock().get_position() + 1;

                if new_position >= -3 && new_position <= 4 {
                    if new_position != ws_opponent.lock().get_position() {
                        ws_player.lock().set_position(new_position);
                    }
                }
            }

            WobbleSquabbleMove::Rebalance => {
                if !ws_player.lock().is_rebalanced() {
                    ws_player.lock().set_rebalanced(true);
                    ws_player.lock().set_balance(0);
                }
            }

            WobbleSquabbleMove::None => {}
        }
    }

    /// Mirrors `endGame(int)`.
    pub fn end_game(&mut self, winner: i32) {
        if !self
            .room
            .get_task_manager()
            .has_task(WobbleSquabbleManager::get_instance().get_name())
        {
            return;
        }

        self.has_game_ended = true;
        let loser = if winner == 1 { 0 } else { 1 };

        {
            let Some(ws_player_0) = self.get_player_arc(0) else {
                return;
            };
            let Some(ws_player_1) = self.get_player_arc(1) else {
                return;
            };

            let first_ip = NettyPlayerNetwork::get_ip_address(
                ws_player_0.lock().get_player().lock().get_network(),
            );
            let second_ip = NettyPlayerNetwork::get_ip_address(
                ws_player_1.lock().get_player().lock().get_network(),
            );

            if first_ip != second_ip {
                if winner != -1 {
                    // Game didn't tie!
                    if let Some(winning_ws_player) = self.get_player_arc(winner) {
                        winning_ws_player
                            .lock()
                            .get_player()
                            .lock()
                            .get_statistic_manager()
                            .increment_value(PlayerStatistic::WobbleSquabbleGamesWon, 1);
                    }
                } else {
                    // Game tied!
                    for i in 0..2 {
                        if let Some(ws_player) = self.get_player_arc(i) {
                            ws_player
                                .lock()
                                .get_player()
                                .lock()
                                .get_statistic_manager()
                                .increment_value(PlayerStatistic::WobbleSquabbleGamesWon, 1);
                        }
                    }
                }

                for i in 0..2 {
                    if let Some(ws_player) = self.get_player_arc(i) {
                        let score = ws_player.lock().get_score();

                        if score > 0 {
                            let ws_guard = ws_player.lock();
                            let player = ws_guard.get_player().lock();
                            player
                                .get_statistic_manager()
                                .increment_value(PlayerStatistic::WobbleSquabbleMonthlyScores, score);
                            player
                                .get_statistic_manager()
                                .increment_value(PlayerStatistic::WobbleSquabblePointsAllTime, score);
                        }
                    }
                }
            }
        }

        // Send end game
        self.send(&PT_WIN::new(winner));

        // Do updates after the players have fallen
        self.end_game_cleanup(winner, loser);
    }

    /// Mirrors the scheduled end-of-game cleanup body.
    fn end_game_cleanup(&mut self, winner: i32, loser: i32) {
        self.send(&PT_END);

        // If it's a tie then remove both
        if winner == -1 {
            self.remove_player(0, true);
            self.remove_player(1, true);
        } else {
            self.remove_player(winner, false);
            self.remove_player(loser, true);
        }

        // Cancel wobble squabble task
        self.room
            .get_task_manager()
            .cancel_task(WobbleSquabbleManager::get_instance().get_name());

        // Make users walk forward
        self.move_queued_users();
    }

    /// Mirrors `moveQueuedUsers()`.
    fn move_queued_users(&mut self) {
        for player in self.room.get_entity_manager().get_players() {
            let player = player.lock();
            let Some(room_user) = player.get_room_user() else {
                continue;
            };

            let Some(item) = room_user.get_current_item() else {
                continue;
            };

            if item.get_definition().get_interaction_type()
                != Some(InteractionType::WsQueueTile)
            {
                continue;
            }

            let position = room_user.get_position();
            let front = position.get_square_in_front();
            room_user.walk_to(front.get_x(), front.get_y());
        }
    }

    /// Mirrors `removePlayer(int, boolean)`.
    pub fn remove_player(&mut self, player_num: i32, is_thrown: bool) {
        let Some(ws_player) = self.get_player_arc(player_num) else {
            return;
        };

        let ws_player = ws_player.lock();
        let balance = ws_player.get_balance();
        let player = ws_player.get_player().lock();
        if let Some(room_user) = player.get_room_user() {
            room_user.set_walking_allowed(true);
        }

        CurrencyDao::decrease_tickets(player.get_details(), 1);
        let tickets = player.get_details().get_tickets();
        player.send(&TICKET_BALANCE::new(tickets));

        if is_thrown {
            if let Some(room_user) = player.get_room_user() {
                room_user.set_status(StatusType::Swim, "");

                let position = room_user.get_position();
                let new_x = position.get_x() + if balance < 0 { -1 } else { 1 };
                let new_y = position.get_y();

                let mut target = Position::new_xy(new_x, new_y);
                target.set_rotation(position.get_rotation());

                room_user.warp(&target, true, false);
            }
        }
    }

    /// Mirrors `getRoom()`.
    pub fn get_room(&self) -> &Room {
        &self.room
    }

    /// Mirrors `getPlayer(int)` (1 or 0 is allowed only; the Java `null` is
    /// `None`).
    pub fn get_player_arc(&self, num: i32) -> Option<Arc<Mutex<WobbleSquabblePlayer>>> {
        [
            Arc::clone(&self.first_player),
            Arc::clone(&self.second_player),
        ]
        .into_iter()
        .find(|ws_player| ws_player.lock().get_order() == num)
    }

    /// Mirrors `getPlayerById(int)`.
    pub fn get_player_by_id(&self, id: i32) -> Option<Arc<Mutex<WobbleSquabblePlayer>>> {
        for i in 0..2 {
            let Some(ws_player) = self.get_player_arc(i) else {
                continue;
            };

            if ws_player
                .lock()
                .get_player()
                .lock()
                .get_details()
                .get_id()
                == id
            {
                return Some(ws_player);
            }
        }

        None
    }
}
