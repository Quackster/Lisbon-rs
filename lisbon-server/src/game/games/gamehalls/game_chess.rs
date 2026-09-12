//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.GameChess`.
//+ Port note: `com.github.bhlangonijr.chesslib` has no Rust equivalent;
// see `chesslib` (inert stubs), so the board state never advances.

use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::gamehalls::chesslib::{Board, Move, MoveGenerator, Piece, PieceType, Side, Square};
use crate::game::games::gamehalls::gamehall_game::{GamehallGame, GamehallGameHandle};
use crate::messages::types::MessageComposer;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::games::item_msg::ITEMMSG;
use crate::messages::outgoing::rooms::user::chat_message::{CHAT_MESSAGE, ChatMessageType};

/// Mirrors the private static nested `GameChess.GameToken` class.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GameToken {
    token: char,
}

impl GameToken {
    /// Mirrors the `GameToken(char)` constructor.
    fn new(token: char) -> Self {
        Self { token }
    }

    /// Mirrors `getToken()`.
    fn get_token(&self) -> char {
        self.token
    }
}

/// Mirrors `GameChess`.
pub struct GameChess {
    base: GamehallGame,
    game_finished: bool,
    board: Option<Board>,
    game_tokens: Vec<GameToken>,
    next_turn: Option<Arc<Mutex<Player>>>,
    player_sides: Vec<(Arc<Mutex<Player>>, GameToken)>,
}

impl GameChess {
    /// Mirrors the `GameChess(List<int[]>)` constructor.
    pub fn new(chair_coordinates: Vec<[i32; 2]>) -> Self {
        Self {
            base: GamehallGame::new(chair_coordinates),
            game_finished: false,
            board: None,
            game_tokens: Vec::new(),
            next_turn: None,
            player_sides: Vec::new(),
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
        self.board = None;
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

            if self.game_finished {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                return;
            }

            player.lock().send(&ITEMMSG::new_commands(&[
                &game_id,
                &format!("SELECTTYPE {}", side_chosen),
            ]));

            let Some(token) = self.get_token(side_chosen).cloned() else {
                return;
            };
            self.player_sides.push((player.clone(), token));

            // Select the other side for the player.
            let other_token = self
                .game_tokens
                .iter()
                .find(|other| other.get_token() != side_chosen)
                .cloned();

            if let Some(other_token) = other_token {
                for other_player in self.base.get_players() {
                    if !Arc::ptr_eq(&other_player, player) {
                        other_player.lock().send(&ITEMMSG::new_commands(&[
                            &game_id,
                            &format!("SELECTTYPE {}", other_token.get_token()),
                        ]));
                        self.player_sides
                            .push((other_player.clone(), other_token.clone()));
                        break;
                    }
                }
            }

            self.restart_map();
            self.broadcast_map();
        }

        if command == "MOVEPIECE" {
            if self
                .next_turn
                .as_ref()
                .map_or(true, |next| !Arc::ptr_eq(next, player))
            {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                self.broadcast_map();
                return;
            }

            if self.game_finished {
                player
                    .lock()
                    .send(&ITEMMSG::new_commands(&[&game_id, "TYPERESERVED"]));
                self.broadcast_map();
                return;
            }

            if (self.base.get_players().len() as i32) < self.get_minimum_people_required() {
                self.broadcast_map();
                return;
            }

            if self.side_of(player).is_none() {
                self.broadcast_map();
                return;
            }

            let Some(from_square) =
                args.first().and_then(|a| Square::value_of(&a.to_uppercase()))
            else {
                return;
            };
            let Some(to_square) = args
                .get(1)
                .and_then(|a| Square::value_of(&a.to_uppercase()))
            else {
                return;
            };

            if from_square == to_square {
                return;
            }

            let Some(board) = self.board.as_mut() else {
                return;
            };

            let mut move_ = Move::new(from_square, to_square);
            let mut is_legal_move = false;

            match MoveGenerator::generate_legal_moves(board) {
                Ok(move_list) => {
                    is_legal_move = move_list.contains(&move_);

                    // Add pawn -> promotion.
                    if !is_legal_move && board.is_promo_rank(board.get_side_to_move(), &move_) {
                        move_ = Move::new_with_piece(
                            from_square,
                            to_square,
                            Piece::make(board.get_side_to_move(), PieceType::Queen),
                        );
                        is_legal_move = move_list.contains(&move_);
                    }
                }
                Err(_) => {}
            }

            if is_legal_move {
                board.do_move(&move_, true);

                if board.is_draw() {
                    self.game_finished = true;
                    self.show_chat("The chess game has ended in a draw");
                    self.broadcast_map();
                    return;
                } else if board.is_stale_mate() {
                    self.game_finished = true;
                    self.show_chat("The chess game has encountered a stalemate");
                    self.broadcast_map();
                    return;
                } else if board.is_mated() {
                    self.game_finished = true;
                    let name = player.lock().get_details().get_name().to_string();
                    self.show_chat(&format!("{} has won the chess game", name));
                    self.broadcast_map();
                    return;
                }

                self.swap_turns(player);
            }

            if let Some(room_user) = player.lock().get_room_user() {
                room_user.reset_room_timer();
            }

            self.broadcast_map();
        }

        if command == "RESTART" {
            self.restart_map();
            self.broadcast_map();
            return;
        }
    }

    /// Send the game map to the opponents.
    fn broadcast_map(&mut self) {
        let mut board_data = String::new();
        let Some(board) = self.board.as_ref() else {
            // Java would throw an NPE on a null board.
            return;
        };

        for square in Square::values() {
            let Some(piece) = board.get_piece(square) else {
                continue;
            };

            if piece.get_piece_type() == PieceType::None || piece.get_piece_side().is_none() {
                continue;
            }

            let side = if piece.get_piece_side() == Some(Side::Black) { "B" } else { "W" };

            board_data.push_str(side);
            board_data.push_str(&self.get_chess_piece(piece.get_piece_type()));
            board_data.push_str(&square.value().to_lowercase());
            board_data.push(' ');
        }

        let player_names = self.get_currently_playing();

        if player_names.len() == 2 {
            self.base
                .send_to_everyone(&Arc::new(ITEMMSG::new_commands(&[
                    self.base.get_game_id().unwrap_or(""),
                    "PIECEDATA",
                    &player_names[0],
                    &player_names[1],
                    &board_data,
                ])));
        }
    }

    /// Get the CCT type of chess piece by the piece type supplied.
    pub fn get_chess_piece(&self, piece_type: PieceType) -> String {
        let chess_piece = match piece_type {
            PieceType::Bishop => "cr",
            PieceType::Knight => "hr",
            PieceType::King => "kg",
            PieceType::Queen => "qu",
            PieceType::Rook => "tw",
            _ => "sd",
        };

        chess_piece.to_string()
    }

    /// Get the name of the user(s) currently playing as an array for the
    /// packet.
    fn get_currently_playing(&self) -> Vec<String> {
        let mut player_names = vec![String::new(), String::new()];

        if let Some(next) = &self.next_turn {
            // A missing side would throw in Java; keep the empty name.
            if let Some(side) = self.side_of(next) {
                player_names[0] = format!(
                    "{} {}",
                    side.get_token().to_uppercase().collect::<String>(),
                    next.lock().get_details().get_name()
                );
            }
        }

        player_names
    }

    /// Mirrors `showChat(String)`.
    fn show_chat(&self, chat: &str) {
        for p in self.base.get_players() {
            let p = p.lock();
            let instance_id = p.get_room_user().map(|ru| ru.get_instance_id()).unwrap_or(0);
            p.send(&CHAT_MESSAGE::new(ChatMessageType::Chat, instance_id, chat));
        }
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
        self.game_tokens = vec![GameToken::new('w'), GameToken::new('b')];

        if !self.base.get_players().is_empty() {
            self.next_turn = self.get_player_by_side('w').cloned();
        }

        self.game_finished = false;
        self.board = Some(Board::new());
    }

    /// Get token instance by character.
    fn get_token(&self, side: char) -> Option<&GameToken> {
        self.game_tokens.iter().find(|t| t.get_token() == side)
    }

    /// Locate a player instance by the side they're playing.
    fn get_player_by_side(&self, side: char) -> Option<&Arc<Mutex<Player>>> {
        self.player_sides
            .iter()
            .find(|(_, token)| token.get_token() == side)
            .map(|(player, _)| player)
    }

    /// Locate the side a player is playing.
    fn side_of(&self, player: &Arc<Mutex<Player>>) -> Option<&GameToken> {
        self.player_sides
            .iter()
            .find(|(p, _)| Arc::ptr_eq(p, player))
            .map(|(_, token)| token)
    }

    /// Mirrors `getMaximumPeopleRequired()`.
    pub fn get_maximum_people_required(&self) -> i32 {
        2
    }

    /// Mirrors `getMinimumPeopleRequired()`.
    pub fn get_minimum_people_required(&self) -> i32 {
        2
    }

    /// Mirrors `getGameFuseType()`.
    pub fn get_game_fuse_type(&self) -> String {
        "Chess".to_string()
    }
}

fn get_trigger(item: &Item) -> Option<&ChessTrigger> {
    item.get_definition()
        .get_interaction_type()
        .and_then(|interaction_type| interaction_type.get_trigger())
        .and_then(|trigger| trigger.downcast_ref::<ChessTrigger>())
}

/// Mirrors the `GamehallGame` upcast for `GameChess`.
impl GamehallGameHandle for GameChess {
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
