//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.GameBattleShip`.
//+ Port note: `GameScheduler.getService().schedule(...)` has no tokio
// equivalent yet; the scheduled turn rotation runs immediately instead.

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::gamehalls::gamehall_game::{GamehallGame, GamehallGameHandle};
use crate::messages::types::MessageComposer;
use crate::game::games::gamehalls::utils::game_ship::GameShip;
use crate::game::games::gamehalls::utils::game_ship_move::GameShipMove;
use crate::game::games::gamehalls::utils::game_ship_move_result::GameShipMoveResult;
use crate::game::games::gamehalls::utils::game_ship_type::GameShipType;
use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::games::item_msg::ITEMMSG;

/// Mirrors `GameBattleShip`.
pub struct GameBattleShip {
    base: GamehallGame,
    ships_placed: HashMap<GameShip, i32>,
    player_list_map: HashMap<i32, Vec<GameShipMove>>,
    players: [Option<Arc<Mutex<Player>>>; 2],
    next_turn: i32,
    is_turn_used: bool,
    game_started: bool,
    //+ Port note: `gameEnded` is write-only in Java.
    game_ended: bool,
}

impl GameBattleShip {
    /// Mirrors the `GameBattleShip(List<int[]>)` constructor.
    pub fn new(chair_coordinates: Vec<[i32; 2]>) -> Self {
        Self {
            base: GamehallGame::new(chair_coordinates),
            ships_placed: HashMap::new(),
            player_list_map: HashMap::new(),
            players: [None, None],
            next_turn: 0,
            is_turn_used: false,
            game_started: false,
            game_ended: false,
        }
    }

    /// Mirrors `gameStart()`.
    pub fn game_start(&mut self) {
        self.ships_placed = HashMap::new();
        self.player_list_map = HashMap::new();
        self.players = [None, None];
        self.next_turn = 0;
        self.is_turn_used = false;
        self.game_started = false;
        self.game_ended = false;
    }

    /// Mirrors `gameStop()`.
    pub fn game_stop(&mut self) {
        self.ships_placed = HashMap::new();
        self.player_list_map = HashMap::new();
        self.players = [None, None];
        self.next_turn = 0;
        self.is_turn_used = false;
        self.game_started = false;
        self.game_ended = false;
    }

    /// Mirrors `joinGame(Player)`.
    pub fn join_game(&mut self, player: &Arc<Mutex<Player>>) {
        if !self.game_started {
            return;
        }

        if self.get_player_num(player) == -1 {
            if self.players[0].is_none() {
                self.players[0] = Some(player.clone());
            } else if self.players[1].is_none() {
                self.players[1] = Some(player.clone());
            }
        }

        self.send_opponents();
        self.send_marked_map();
    }

    /// Mirrors `leaveGame(Player)`.
    pub fn leave_game(&mut self, player: &Arc<Mutex<Player>>) {
        if self.next_turn == self.get_player_num(player) {
            self.rotate_turn();
        }

        for slot in self.players.iter_mut() {
            if let Some(p) = slot {
                if Arc::ptr_eq(p, player) {
                    *slot = None;
                }
            }
        }
    }

    /// Mirrors `handleCommand(Player, Room, Item, String, String[])`.
    pub fn handle_command(
        &mut self,
        player: &Arc<Mutex<Player>>,
        _room: &Room,
        item: &Item,
        command: &str,
        args: &[String],
    ) {
        let player_num = self.get_player_num(player);

        if command == "PLACESHIP" {
            if self.game_started {
                return;
            }

            let (ship_id, start_x, start_y, _end_x, end_y) = match (
                args.get(0).and_then(|a| a.parse::<i32>().ok()),
                args.get(1).and_then(|a| a.parse::<i32>().ok()),
                args.get(2).and_then(|a| a.parse::<i32>().ok()),
                args.get(3).and_then(|a| a.parse::<i32>().ok()),
                args.get(4).and_then(|a| a.parse::<i32>().ok()),
            ) {
                (Some(a), Some(b), Some(c), Some(d), Some(e)) => (a, b, c, d, e),
                _ => return,
            };

            let Some(ship_type) = GameShipType::get_by_id(ship_id) else {
                return;
            };

            if player_num == -1 {
                if self.players[0].is_none() {
                    self.players[0] = Some(player.clone());
                } else if self.players[1].is_none() {
                    self.players[1] = Some(player.clone());
                }
            }

            if self.count_ships(Some(ship_type), player_num) >= ship_type.get_max_allowed() {
                return;
            }

            self.player_list_map
                .entry(player_num)
                .or_insert_with(Vec::new);

            let is_horizontal = start_y == end_y;

            self.ships_placed.insert(
                GameShip::new(
                    ship_type,
                    Position::new_xy(start_x, start_y),
                    player_num,
                    is_horizontal,
                ),
                player_num,
            );

            if self.has_everyone_finished() {
                self.send_opponents();
                self.rotate_turn();
                self.game_started = true;
            }
        }

        if command == "SHOOT" {
            if self.next_turn != player_num {
                return;
            }

            if self.is_turn_used {
                return;
            }

            let (x, y) = match (
                args.get(0).and_then(|a| a.parse::<i32>().ok()),
                args.get(1).and_then(|a| a.parse::<i32>().ok()),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => return,
            };

            let move_result = if self.is_hit(x, y, player_num) {
                GameShipMoveResult::Hit
            } else {
                GameShipMoveResult::Miss
            };

            let Some(opposite_player) = self.get_opposite_player(player) else {
                return;
            };

            let game_ship = self.get_ship_placed(x, y, player_num);

            if let Some(moves) = self.player_list_map.get_mut(&player_num) {
                moves.push(GameShipMove::new(
                    player.clone(),
                    x,
                    y,
                    move_result,
                    game_ship.clone(),
                ));
            }

            self.send_marked_map();

            match &game_ship {
                Some(ship) if ship.is_destroyed(self) => {
                    self.send_to_all("SINK");
                }
                Some(ship) if ship.is_hit_twice(self) => {
                    self.send_to_all("HITTWICE");
                }
                Some(_) => {
                    self.send_to_all("HIT");
                }
                None => {
                    self.send_to_all("MISS");
                }
            }

            if move_result == GameShipMoveResult::Miss {
                self.is_turn_used = true;
                //+ Port note: `GameScheduler.getService().schedule(rotateTurn, 2, SECONDS)`.
                self.rotate_turn();
            } else {
                self.is_turn_used = false;
                self.send_to_turn();
            }

            if self.is_game_over(&opposite_player) {
                self.game_ended = true;
                let name = player.lock().get_details().get_name().to_string();
                self.base
                    .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                        self.base.get_game_id().unwrap_or(""),
                        "GAMEEND",
                        &name,
                    ])));
                self.base
                    .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                        self.base.get_game_id().unwrap_or(""),
                        "GAMEOVER",
                    ])));
                self.is_turn_used = true;
            }

            if let Some(room_user) = player.lock().get_room_user() {
                room_user.reset_room_timer();
            }
        }

        if command == "CLOSE" {
            let player_ref = player.lock();
            let entity: &dyn Entity = &*player_ref;

            // The Java NPEs when the trigger is missing.
            if let Some(trigger) = get_trigger(item) {
                if let Some(room_user) = entity.get_room_user() {
                    trigger.on_entity_leave(entity, room_user, item);
                }
            }

            return;
        }
    }

    /// Mirrors `rotateTurn()`.
    fn rotate_turn(&mut self) {
        self.next_turn = self.get_opposite_player_num(self.next_turn);
        self.send_to_turn();
        self.is_turn_used = false;
    }

    fn send_to_turn(&mut self) {
        self.base
            .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                self.base.get_game_id().unwrap_or(""),
                "TURN",
                &self.next_turn.to_string(),
            ])));
    }

    fn send_to_all(&mut self, command: &str) {
        self.base
            .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                self.base.get_game_id().unwrap_or(""),
                command,
            ])));
    }

    /// Mirrors the `OPPONENTS` packet block shared by `joinGame` and
    /// `handleCommand`.
    fn send_opponents(&mut self) {
        let data = self.build_opponents();
        self.base
            .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                self.base.get_game_id().unwrap_or(""),
                "OPPONENTS",
                &data.join("\r"),
            ])));
    }

    fn build_opponents(&self) -> [String; 2] {
        let mut opponent_data = [String::new(), String::new()];

        for i in 0..2 {
            let name = match &self.players[i] {
                Some(p) => p.lock().get_details().get_name().to_string(),
                None => String::new(),
            };
            let num = match &self.players[i] {
                Some(p) => self.get_player_num(p),
                None => i as i32,
            };
            opponent_data[i] = format!("{} {}", num, name);
        }

        opponent_data
    }

    /// Mirrors `sendMarkedMap()`.
    fn send_marked_map(&self) {
        for slot in &self.players {
            let Some(p) = slot else {
                continue;
            };

            if self.get_opposite_player(p).is_none() {
                return;
            }

            let opponent_num = self.get_opposite_player_num(self.get_player_num(p));

            let msg = if self.get_player_num(p) == 0 {
                ITEMMSG::new_commands(&[
                    self.base.get_game_id().unwrap_or(""),
                    "SITUATION",
                    "",
                    &self.generate_hit_grid(self.get_player_num(p)),
                    "",
                    &self.generate_hit_grid(opponent_num),
                ])
            } else {
                ITEMMSG::new_commands(&[
                    self.base.get_game_id().unwrap_or(""),
                    "SITUATION",
                    "",
                    &self.generate_hit_grid(opponent_num),
                    "",
                    &self.generate_hit_grid(self.get_player_num(p)),
                ])
            };

            p.lock().send(&msg);
        }
    }

    /// Mirrors `generateHitGrid(int)`.
    fn generate_hit_grid(&self, player: i32) -> String {
        let mut map = String::new();

        for y in 0..12 {
            for x in 0..13 {
                let ship_move = self
                    .player_list_map
                    .get(&self.get_opposite_player_num(player))
                    .and_then(|moves| {
                        moves
                            .iter()
                            .find(|move_| move_.get_x() == x && move_.get_y() == y)
                    });

                match ship_move {
                    None => map.push('-'),
                    Some(move_) => {
                        if let Some(ship) = move_.get_ship() {
                            if ship.is_destroyed(self) {
                                map.push_str(GameShipMoveResult::Sink.get_symbol());
                            } else {
                                map.push_str(move_.get_move_result().get_symbol());
                            }
                        } else {
                            map.push_str(move_.get_move_result().get_symbol());
                        }
                    }
                }
            }
        }

        map
    }

    /// Mirrors `isHit(int, int, int)`.
    fn is_hit(&self, select_x: i32, select_y: i32, player: i32) -> bool {
        self.get_ship_placed(select_x, select_y, player).is_some()
    }

    /// Mirrors `getShipPlaced(int, int, int)`.
    fn get_ship_placed(&self, x: i32, y: i32, player: i32) -> Option<GameShip> {
        let opponent_num = self.get_opposite_player_num(player);

        for (ship, owner) in &self.ships_placed {
            if *owner != opponent_num {
                continue;
            }

            for i in 0..ship.get_ship_type().get_length() {
                let ship_x = ship.get_position().get_x() + if ship.is_horizontal() { i } else { 0 };
                let ship_y = ship.get_position().get_y() + if ship.is_horizontal() { 0 } else { i };

                if x == ship_x && y == ship_y {
                    return Some(ship.clone());
                }
            }
        }

        None
    }

    /// Get the player number for the player.
    pub fn get_player_num(&self, player: &Arc<Mutex<Player>>) -> i32 {
        for (i, slot) in self.players.iter().enumerate() {
            if let Some(p) = slot {
                if Arc::ptr_eq(p, player) {
                    return i as i32;
                }
            }
        }

        -1
    }

    /// Mirrors `isGameOver(Player)`.
    fn is_game_over(&self, player: &Arc<Mutex<Player>>) -> bool {
        let num = self.get_player_num(player);

        for (ship, owner) in &self.ships_placed {
            if *owner != num {
                continue;
            }

            if !ship.is_destroyed(self) {
                return false;
            }
        }

        true
    }

    /// Get the opposite player playing.
    pub fn get_opposite_player(
        &self,
        player: &Arc<Mutex<Player>>,
    ) -> Option<Arc<Mutex<Player>>> {
        for slot in &self.players {
            if let Some(p) = slot {
                if !Arc::ptr_eq(p, player) {
                    return Some(p.clone());
                }
            }
        }

        None
    }

    /// Mirrors `getOppositePlayerNum(int)`.
    pub fn get_opposite_player_num(&self, player: i32) -> i32 {
        if player == 0 {
            1
        } else {
            0
        }
    }

    /// Gets if both players have finished placing their pieces.
    fn has_everyone_finished(&self) -> bool {
        if self.count_ships(None, 0) != 10 {
            return false;
        }

        if self.count_ships(None, 1) != 10 {
            return false;
        }

        true
    }

    /// Count the ships placed on the map.
    fn count_ships(&self, ship_type: Option<GameShipType>, player: i32) -> i32 {
        self.ships_placed
            .iter()
            .filter(|(ship, _)| {
                let type_ok = match ship_type {
                    Some(st) => ship.get_ship_type() == st,
                    None => true,
                };
                let player_ok = if player != -1 {
                    ship.get_player() == player
                } else {
                    true
                };

                type_ok && player_ok
            })
            .count() as i32
    }

    /// Mirrors `getMaximumPeopleRequired()`.
    pub fn get_maximum_people_required(&self) -> i32 {
        2
    }

    /// Mirrors `getMinimumPeopleRequired()`.
    pub fn get_minimum_people_required(&self) -> i32 {
        1
    }

    /// Mirrors `getGameFuseType()`.
    pub fn get_game_fuse_type(&self) -> String {
        "BattleShip".to_string()
    }

    /// Mirrors `getPlayerListMap()`.
    pub fn get_player_list_map(&self) -> &HashMap<i32, Vec<GameShipMove>> {
        &self.player_list_map
    }
}

fn get_trigger(item: &Item) -> Option<&BattleShipsTrigger> {
    item.get_definition()
        .get_interaction_type()
        .and_then(|interaction_type| interaction_type.get_trigger())
        .and_then(|trigger| trigger.downcast_ref::<BattleShipsTrigger>())
}

/// Mirrors the `GamehallGame` upcast for `GameBattleShip`.
impl GamehallGameHandle for GameBattleShip {
    fn base(&self) -> &GamehallGame {
        &self.base
    }

    fn get_game_id(&self) -> Option<String> {
        self.base.get_game_id().map(|id| id.to_string())
    }

    fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
        self.base.get_players()
    }

    fn refresh_players(&mut self) -> Vec<Arc<Mutex<Player>>> {
        self.base.refresh_players()
    }

    fn create_game_id(&mut self) {
        self.base.create_game_id();
    }

    fn reset_game_id(&mut self) {
        self.base.reset_game_id();
    }

    fn set_room_id(&mut self, room_id: i32) {
        self.base.set_room_id(room_id);
    }

    fn has_players_required(&self) -> bool {
        // The Java `hasPlayersRequired()` resolves `getMinimumPeopleRequired()`
        // virtually; compute it directly against the concrete minimum.
        self.base.get_players().len() as i32 >= self.get_minimum_people_required()
    }

    fn remove_player(&mut self, player: &Arc<Mutex<Player>>) {
        self.base.remove_player(player);
    }

    fn game_start(&mut self) {
        self.game_start();
    }

    fn game_stop(&mut self) {
        self.game_stop();
    }

    fn get_game_fuse_type(&self) -> String {
        self.get_game_fuse_type()
    }

    fn get_minimum_people_required(&self) -> i32 {
        self.get_minimum_people_required()
    }

    fn get_maximum_people_required(&self) -> i32 {
        self.get_maximum_people_required()
    }

    fn handle_command(
        &mut self,
        player: &Arc<Mutex<Player>>,
        room: &Room,
        item: &Item,
        command: &str,
        args: &[String],
    ) {
        self.handle_command(player, room, item, command, args);
    }

    fn send_to_everyone(&mut self, message: &Arc<dyn MessageComposer + Send + Sync>) {
        self.base.send_to_everyone_composer(message);
    }
}
