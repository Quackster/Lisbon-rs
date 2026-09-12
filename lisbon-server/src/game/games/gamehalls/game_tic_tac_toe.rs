//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.GameTicTacToe`.

use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::gamehalls::gamehall_game::{GamehallGame, GamehallGameHandle};
use crate::messages::types::MessageComposer;
use crate::game::games::gamehalls::utils::game_token::GameToken;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::games::close_game_board::CLOSEGAMEBOARD;
use crate::messages::outgoing::rooms::games::item_msg::ITEMMSG;
use crate::messages::outgoing::rooms::user::chat_message::{CHAT_MESSAGE, ChatMessageType};

/// Mirrors the `NUM_IN_ROW` constant.
const NUM_IN_ROW: i32 = 5;
/// Mirrors the `MAX_WIDTH` constant.
const MAX_WIDTH: i32 = 23;
/// Mirrors the `MAX_LENGTH` constant.
const MAX_LENGTH: i32 = 24;

/// Mirrors `GameTicTacToe`.
pub struct GameTicTacToe {
    base: GamehallGame,
    game_tokens: Vec<GameToken>,
    player_sides: Vec<(Arc<Mutex<Player>>, char)>,
    game_finished: bool,
    game_map: Option<Vec<[char; MAX_LENGTH as usize]>>,
    next_turn: Option<Arc<Mutex<Player>>>,
}

impl GameTicTacToe {
    /// Mirrors the `GameTicTacToe(List<int[]>)` constructor.
    pub fn new(chair_coordinates: Vec<[i32; 2]>) -> Self {
        Self {
            base: GamehallGame::new(chair_coordinates),
            game_tokens: Vec::new(),
            player_sides: Vec::new(),
            game_finished: false,
            game_map: None,
            next_turn: None,
        }
    }

    /// Mirrors `gameStart()`.
    pub fn game_start(&mut self) {
        self.player_sides.clear();
        self.restart_map();
    }

    /// Mirrors `gameStop()`.
    pub fn game_stop(&mut self) {
        self.player_sides.clear();
        self.game_map = None;
    }

    /// Mirrors `joinGame(Player)`.
    pub fn join_game(&mut self, _player: &Arc<Mutex<Player>>) {}

    /// Mirrors `leaveGame(Player)`.
    pub fn leave_game(&mut self, player: &Arc<Mutex<Player>>) {
        self.player_sides
            .retain(|(p, _)| !Arc::ptr_eq(p, player));
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
        let game_id = self.base.get_game_id().unwrap_or("").to_string();

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

        if command == "CHOOSETYPE" {
            let Some(side_chosen) = args.first().and_then(|a| a.chars().next()) else {
                return;
            };

            if self.get_token(side_chosen).is_none() {
                return;
            }

            if self.get_player_by_side(side_chosen).is_some() {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                return;
            }

            let side = side_chosen;
            self.player_sides.push((player.clone(), side));

            player.lock().send(&ITEMMSG::new_commands(&[
                &game_id,
                &format!("SELECTTYPE {}", side),
            ]));

            // Select the other side for the player.
            let other_token = self
                .game_tokens
                .iter()
                .find(|other| other.get_token() != side)
                .map(|other| other.get_token());

            if let Some(other_side) = other_token {
                for other_player in self.base.get_players() {
                    if !Arc::ptr_eq(&other_player, player) {
                        other_player.lock().send(&ITEMMSG::new_commands(&[
                            &game_id,
                            &format!("SELECTTYPE {}", other_side),
                        ]));
                        self.player_sides
                            .push((other_player.clone(), other_side));
                        break;
                    }
                }
            }

            let player_names = self.get_currently_playing();

            if !player_names.is_empty() {
                self.base
                    .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                        &game_id,
                        "OPPONENTS",
                        &player_names[0],
                        &player_names[1],
                    ])));
            } else {
                player.lock().send(&CLOSEGAMEBOARD::new(
                    self.base.get_game_id().unwrap_or(""),
                    &self.get_game_fuse_type(),
                ));
                self.base.remove_player(player);
                self.base.reset_game_id();
                self.game_stop();
            }
        }

        if command == "RESTART" {
            self.restart_map();
            self.broadcast_map();
            return;
        }

        if command == "SETSECTOR" {
            if (self.base.get_players().len() as i32) < self.get_minimum_people_required() {
                return;
            }

            if self.side_of(player).is_none() {
                return;
            }

            if self
                .next_turn
                .as_ref()
                .map_or(true, |next| !Arc::ptr_eq(next, player))
            {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                return;
            }

            if self.game_finished {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                return;
            }

            let Some(side) = args.first().and_then(|a| a.chars().next()) else {
                return;
            };

            if self.side_of(player) != Some(side) {
                return;
            }

            let (x, y) = match (
                args.get(2).and_then(|a| a.parse::<i32>().ok()),
                args.get(1).and_then(|a| a.parse::<i32>().ok()),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => return,
            };

            if x >= MAX_WIDTH || y >= MAX_LENGTH || x < 0 || y < 0 {
                return;
            }

            if self.game_map.is_none() {
                return;
            }

            if self.game_map.as_ref().unwrap()[x as usize][y as usize] != '0' {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                return;
            }

            let Some(token) = self
                .game_tokens
                .iter_mut()
                .find(|t| t.get_token() == side)
            else {
                return;
            };
            token.increment_moves();
            let placed = token.get_token();
            self.game_map
                .as_mut()
                .unwrap()
                [x as usize][y as usize] = placed;

            if let Some(variables) = self.has_game_finished() {
                self.game_finished = true;
                self.announce_winning_side(&variables);
            } else {
                self.swap_turns(player);
            }

            if let Some(room_user) = player.lock().get_room_user() {
                room_user.reset_room_timer();
            }

            self.broadcast_map();
        }
    }

    /// Announce the winning side, change the characters to their winning
    /// symbols, and say how many moves it took.
    fn announce_winning_side(&mut self, variables: &(char, Vec<[i32; 2]>)) {
        let Some(token) = self
            .game_tokens
            .iter()
            .find(|side| side.get_token() == variables.0)
        else {
            return;
        };

        let moves = token.get_moves();
        let winning_token = token.get_winning_token();
        let side = token.get_token();

        for coord in &variables.1 {
            self.game_map
                .as_mut()
                .unwrap()
                [coord[0] as usize][coord[1] as usize] = winning_token;
        }

        self.broadcast_map();

        let Some(winner) = self.get_player_by_side(side) else {
            return;
        };

        let winner_name = winner.lock().get_details().get_name().to_string();

        for p in self.base.get_players() {
            let p = p.lock();
            let instance_id = p.get_room_user().map(|ru| ru.get_instance_id()).unwrap_or(0);
            p.send(&CHAT_MESSAGE::new(
                ChatMessageType::Chat,
                instance_id,
                &format!("{} has won the game in {} moves", winner_name, moves),
            ));
        }
    }

    /// Check for the winner.
    fn has_game_finished(&self) -> Option<(char, Vec<[i32; 2]>)> {
        let Some(map) = self.game_map.as_ref() else {
            return None;
        };
        let mut winning_coordinates: Vec<[i32; 2]> = Vec::new();

        // Check rows across.
        for i in 0..MAX_WIDTH {
            for j in 0..MAX_LENGTH {
                let letter = map[i as usize][j as usize];
                winning_coordinates.clear();

                if letter == '0' {
                    continue;
                }

                for k in 0..NUM_IN_ROW {
                    if j + k >= MAX_LENGTH {
                        continue;
                    }

                    let new_letter = map[i as usize][(j + k) as usize];

                    if new_letter != '0' && new_letter == letter {
                        winning_coordinates.push([i, j + k]);

                        if (winning_coordinates.len() as i32) >= NUM_IN_ROW {
                            return Some((new_letter, winning_coordinates));
                        }
                    } else {
                        winning_coordinates.clear();
                    }
                }
            }
        }

        // Check rows down.
        for i in 0..MAX_WIDTH {
            for j in 0..MAX_LENGTH {
                let letter = map[i as usize][j as usize];
                winning_coordinates.clear();

                if letter == '0' {
                    continue;
                }

                for k in 0..NUM_IN_ROW {
                    if i + k >= MAX_WIDTH {
                        continue;
                    }

                    let new_letter = map[(i + k) as usize][j as usize];

                    if new_letter != '0' && new_letter == letter {
                        winning_coordinates.push([i + k, j]);

                        if (winning_coordinates.len() as i32) >= NUM_IN_ROW {
                            return Some((new_letter, winning_coordinates));
                        }
                    } else {
                        winning_coordinates.clear();
                    }
                }
            }
        }

        // Check top left to bottom right.
        for i in 0..MAX_WIDTH {
            for j in 0..MAX_LENGTH {
                let letter = map[i as usize][j as usize];
                winning_coordinates.clear();

                if letter == '0' {
                    continue;
                }

                for k in 0..NUM_IN_ROW {
                    // Mirrors the Java bound quirk (`MAX_WIDTH` for `j`).
                    if i + k >= MAX_WIDTH || j + k >= MAX_WIDTH {
                        continue;
                    }

                    let new_letter = map[(i + k) as usize][(j + k) as usize];

                    if new_letter != '0' && new_letter == letter {
                        winning_coordinates.push([i + k, j + k]);

                        if (winning_coordinates.len() as i32) >= NUM_IN_ROW {
                            return Some((new_letter, winning_coordinates));
                        }
                    } else {
                        winning_coordinates.clear();
                    }
                }
            }
        }

        // Check top right to bottom left.
        for i in 0..MAX_WIDTH {
            for j in 0..MAX_LENGTH {
                let letter = map[i as usize][j as usize];
                winning_coordinates.clear();

                if letter == '0' {
                    continue;
                }

                for k in 0..NUM_IN_ROW {
                    let new_x = i - k;
                    let new_y = j + k;

                    if new_x < 0 {
                        continue;
                    }

                    // Mirrors the Java bound quirk (`MAX_WIDTH` for `newY`).
                    if new_x >= MAX_WIDTH || new_y >= MAX_WIDTH {
                        continue;
                    }

                    let new_letter = map[new_x as usize][new_y as usize];

                    if new_letter != '0' && new_letter == letter {
                        winning_coordinates.push([new_x, new_y]);

                        if (winning_coordinates.len() as i32) >= NUM_IN_ROW {
                            return Some((new_letter, winning_coordinates));
                        }
                    } else {
                        winning_coordinates.clear();
                    }
                }
            }
        }

        None
    }

    /// Swap who's turn it is to play.
    fn swap_turns(&mut self, player: &Arc<Mutex<Player>>) {
        let mut next_player = None;

        if self
            .next_turn
            .as_ref()
            .map_or(false, |next| Arc::ptr_eq(next, player))
        {
            for p in self.base.get_players() {
                if !Arc::ptr_eq(&p, player) {
                    next_player = Some(p);
                }
            }
        }

        self.next_turn = next_player;
    }

    /// Reset the game map.
    fn restart_map(&mut self) {
        self.game_tokens = vec![GameToken::new('O', 'q'), GameToken::new('X', '+')];

        let players = self.base.get_players();
        if !players.is_empty() {
            self.next_turn = players.into_iter().next();
        }

        self.game_finished = false;
        self.game_map = Some(vec![[b'0' as char; MAX_LENGTH as usize]; MAX_WIDTH as usize]);
    }

    /// Send the game map to the opponents.
    fn broadcast_map(&mut self) {
        let mut board_data = String::new();

        if let Some(map) = &self.game_map {
            for row in map {
                for map_letter in row {
                    board_data.push(if *map_letter == '0' { ' ' } else { *map_letter });
                }
                board_data.push(' ');
            }
        }

        let player_names = self.get_currently_playing();

        if !player_names.is_empty() {
            self.base
                .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                    self.base.get_game_id().unwrap_or(""),
                    "BOARDDATA",
                    &player_names[0],
                    "",
                    &board_data,
                ])));
        }
    }

    /// Get the name of the user(s) currently playing as an array for the
    /// packet (the Java `try/catch` falls back to an empty array; the
    /// missing-side case does the same here).
    fn get_currently_playing(&self) -> Vec<String> {
        let mut player_names = vec![String::new(), String::new()];

        if let Some(next) = &self.next_turn {
            match self.side_of(next) {
                Some(side) => {
                    player_names[0] = format!(
                        "{} {}",
                        side.to_uppercase().collect::<String>(),
                        next.lock().get_details().get_name()
                    );
                }
                None => return Vec::new(),
            }
        }

        player_names
    }

    /// Locate a player instance by the side they're playing.
    fn get_player_by_side(&self, side: char) -> Option<&Arc<Mutex<Player>>> {
        self.player_sides
            .iter()
            .find(|(_, token)| *token == side)
            .map(|(player, _)| player)
    }

    /// Get token instance by character.
    fn get_token(&self, side: char) -> Option<&GameToken> {
        self.game_tokens.iter().find(|t| t.get_token() == side)
    }

    /// Locate the side a player is playing.
    fn side_of(&self, player: &Arc<Mutex<Player>>) -> Option<char> {
        self.player_sides
            .iter()
            .find(|(p, _)| Arc::ptr_eq(p, player))
            .map(|(_, token)| *token)
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
        "TicTacToe".to_string()
    }
}

fn get_trigger(item: &Item) -> Option<&TicTacToeTrigger> {
    item.get_definition()
        .get_interaction_type()
        .and_then(|interaction_type| interaction_type.get_trigger())
        .and_then(|trigger| trigger.downcast_ref::<TicTacToeTrigger>())
}

/// Mirrors the `GamehallGame` upcast for `GameTicTacToe`.
impl GamehallGameHandle for GameTicTacToe {
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
