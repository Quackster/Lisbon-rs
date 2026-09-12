//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.GamehallGame`.
//!
//! Rust has no inheritance; the concrete gamehall games
//! (`GameChess`, `GamePoker`, `GameBattleShip`, `GameTicTacToe`)
//! embed this base. The Java abstract methods (`gameStart`,
//! `gameStop`, `handleCommand`, `joinGame`, `leaveGame`,
//! `getGameFuseType`, `getMinimumPeopleRequired`,
//! `getMaximumPeopleRequired`) live on the concrete types and are
//! `unimplemented!()` on the base (which never implements them).

use parking_lot::Mutex;
use rand::Rng;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::triggers::game_trigger::GameTrigger;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageComposer;

/// Mirrors the abstract `GamehallGame` class.
pub struct GamehallGame {
    chair_coordinates: Vec<[i32; 2]>,
    players: Vec<Arc<Mutex<Player>>>,
    room_id: i32,
    game_id: Option<String>,
    last_message: Option<Arc<dyn MessageComposer + Send + Sync>>,
}

impl GamehallGame {
    /// Mirrors the `GamehallGame(List<int[]>)` constructor (`CopyOnWriteArrayList` is a plain `Vec`).
    pub fn new(chair_coordinates: Vec<[i32; 2]>) -> Self {
        Self {
            chair_coordinates,
            players: Vec::new(),
            room_id: 0,
            game_id: None,
            last_message: None,
        }
    }

    /// Mirrors `gameStart()`.
    pub fn game_start(&mut self) {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Mirrors `gameStop()`.
    pub fn game_stop(&mut self) {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Mirrors `handleCommand(Player, Room, Item, String, String[])`.
    pub fn handle_command(
        &mut self,
        _player: &Arc<Mutex<Player>>,
        _room: &Room,
        _item: &Item,
        _command: &str,
        _args: &[String],
    ) {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Mirrors `joinGame(Player)`.
    pub fn join_game(&mut self, _player: &Arc<Mutex<Player>>) {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Mirrors `leaveGame(Player)`.
    pub fn leave_game(&mut self, _player: &Arc<Mutex<Player>>) {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Get the unique game ID instance for this pair (null before the
    /// game has initialised).
    pub fn get_game_id(&self) -> Option<&str> {
        self.game_id.as_deref()
    }

    /// Generate the unique game ID instance for this pair.
    pub fn create_game_id(&mut self) {
        const ALPHABET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijlmnopqrstuvwyz1234567890";
        let mut rng = rand::thread_rng();
        let game_id: String = (0..6)
            .map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char)
            .collect();

        self.game_id = Some(game_id);

        for player in &self.players {
            if let Some(room_user) = player.lock().get_room_user() {
                room_user.set_current_game_id(self.game_id.clone());
            }
        }
    }

    /// Resets the game ID back to null.
    pub fn reset_game_id(&mut self) {
        self.game_id = None;
    }

    /// Get the room instance this game instance is running in.
    pub fn get_room(&self) -> Option<Arc<Mutex<Room>>> {
        RoomManager::get_instance().get_room_by_id(self.room_id)
    }

    /// Get the opponents (not including the user supplied).
    pub fn get_opponents(
        &self,
        player: &Arc<Mutex<Player>>,
    ) -> Vec<Arc<Mutex<Player>>> {
        let player_id = player.lock().get_details().get_id();

        self.players
            .iter()
            .filter(|p| p.lock().get_details().get_id() != player_id)
            .cloned()
            .collect()
    }

    /// Send a packet to all opponents except the user supplied.
    pub fn send_to_opponents(
        &self,
        player: &Arc<Mutex<Player>>,
        message: &dyn MessageComposer,
    ) {
        for p in self.get_opponents(player) {
            p.lock().send(message);
        }
    }

    /// Send a packet to everyone playing (trait-object entry point; the
    /// trait `GamehallGameHandle` funnels into it).
    pub fn send_to_everyone_composer(
        &mut self,
        message: &Arc<dyn MessageComposer + Send + Sync>,
    ) {
        self.last_message = Some(message.clone());

        for p in &self.players {
            p.lock().send(message.as_ref());
        }
    }

    /// Send a packet to everyone playing.
    pub fn send_to_everyone<M: MessageComposer + Send + Sync + 'static>(
        &mut self,
        message: &Arc<M>,
    ) {
        // `Arc` has no implicit `&Arc<T> -> &Arc<dyn Trait>` argument
        // coercion; the explicit annotation provides the expected type.
        let message: Arc<dyn MessageComposer + Send + Sync> = message.clone();
        self.send_to_everyone_composer(&message);
    }

    /// Get the list of players at each table (a clone of the handle list;
    /// the Java list alias is not expressible in Rust).
    pub fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
        self.players.clone()
    }

    /// Remove a player from the player list (mirrors the Java
    /// `getPlayers().remove(player)` usage).
    pub fn remove_player(&mut self, player: &Arc<Mutex<Player>>) {
        self.players.retain(|p| !Arc::ptr_eq(p, player));
    }

    /// Get if the server has the correct amount of players required before
    /// the game starts.
    pub fn has_players_required(&self) -> bool {
        self.players.len() as i32 >= self.get_minimum_people_required()
    }

    /// Refresh players currently playing.
    pub fn refresh_players(&mut self) -> Vec<Arc<Mutex<Player>>> {
        let tile_positions: Vec<Position> = self
            .get_tiles()
            .iter()
            .map(|tile| tile.lock().get_position().copy())
            .collect();

        self.players.retain(|p| {
            let player = p.lock();
            let Some(room_user) = player.get_room_user() else {
                return false;
            };
            let position = room_user.get_position();

            room_user.get_room().is_some()
                && tile_positions.iter().any(|tile_position| {
                    tile_position.get_x() == position.get_x()
                        && tile_position.get_y() == position.get_y()
                        && tile_position.get_z() == position.get_z()
                        && tile_position.get_head_rotation()
                            == position.get_head_rotation()
                        && tile_position.get_body_rotation()
                            == position.get_body_rotation()
                })
        });

        let mut new_players: Vec<Arc<Mutex<Player>>> = Vec::new();

        // The room is resolved for the `Arc<Mutex<Player>>` lookups.
        let room = self.get_room();

        for tile in self.get_tiles() {
            let tile_guard = tile.lock();
            let entities = tile_guard.get_entities();

            if entities.is_empty() {
                continue;
            }

            let entity = &entities[0];

            if entity.get_type() != crate::game::entity::entity_type::EntityType::Player {
                continue;
            }

            let Some(player) = entity.as_player() else {
                continue;
            };

            let Some(room_user) = player.get_room_user() else {
                continue;
            };

            if room_user.get_current_game_id().is_some() {
                continue;
            }

            let player_id = player.get_details().get_id();

            if self.players.iter().any(|p| {
                p.lock()
                    .get_details()
                    .get_id() == player_id
            }) {
                continue;
            }

            // The Java `players.add(player)` needs the owning
            // `Arc<Mutex<Player>>`; it is resolved back via the room
            // entity manager.
            let Some(room) = &room else {
                continue;
            };

            let player_arc = room
                .lock()
                .get_entity_manager()
                .get_players()
                .into_iter()
                .find(|p| p.lock().get_details().get_id() == player_id);

            let Some(player_arc) = player_arc else {
                continue;
            };

            player_arc
                .lock()
                .get_room_user()
                .map(|room_user| {
                    room_user.set_current_game_id(self.game_id.clone());
                });

            self.players.push(Arc::clone(&player_arc));
            new_players.push(player_arc);
        }

        new_players
    }

    /// Return the room tiles for this room (the shared tile handles; the
    /// Java `List<RoomTile>` cannot be owned past the room lock in Rust).
    pub fn get_tiles(&self) -> Vec<Arc<Mutex<crate::game::room::mapping::room_tile::RoomTile>>> {
        let mut tiles = Vec::new();
        let Some(room) = self.get_room() else {
            return tiles;
        };
        let room = room.lock();

        for coord in &self.chair_coordinates {
            if let Some(tile) = room.get_mapping().lock().get_tile_handle(coord[0], coord[1]) {
                tiles.push(tile);
            }
        }

        tiles
    }

    /// If this position is invalid, as in, the position is a chair to play on.
    fn has_position(&self, position: &Position) -> bool {
        for tile in self.get_tiles() {
            let tile = tile.lock();
            let tile_position = tile.get_position();

            if tile_position.get_x() == position.get_x()
                && tile_position.get_y() == position.get_y()
                && tile_position.get_z() == position.get_z()
                && tile_position.get_head_rotation() == position.get_head_rotation()
                && tile_position.get_body_rotation() == position.get_body_rotation()
            {
                return true;
            }
        }

        false
    }

    /// Get FUSE game type.
    pub fn get_game_fuse_type(&self) -> String {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Get the minimum people required for a game to start.
    pub fn get_minimum_people_required(&self) -> i32 {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Get the maximum people required before no one else is allowed to join.
    pub fn get_maximum_people_required(&self) -> i32 {
        unimplemented!("abstract method; overridden by the game implementations")
    }

    /// Restarts the game.
    pub fn restart_game(&self, trigger: &GameTrigger, item: &Item) {
        for player in &self.players {
            let player = player.lock();
            let entity: &dyn Entity = &*player;

            if let Some(room_user) = entity.get_room_user() {
                trigger.on_entity_leave(entity, room_user, item);
            }
        }
    }

    /// Get the room id this gamehall game is in.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Set the room id this gamehall game is in.
    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    /// Return the list of chair coordinates.
    pub fn get_chair_coordinates(&self) -> &[[i32; 2]] {
        &self.chair_coordinates
    }

    /// Get the last message sent.
    pub fn get_last_message(&self) -> Option<&Arc<dyn MessageComposer + Send + Sync>> {
        self.last_message.as_ref()
    }
}

/// Mirrors the `GamehallGame` upcast: the concrete game-hall games
/// (`GameChess`, `GamePoker`, `GameBattleShip`, `GameTicTacToe`) share
/// this handle interface (Rust has no inheritance).
pub trait GamehallGameHandle: Send {
    fn base(&self) -> &GamehallGame;

    fn get_game_id(&self) -> Option<String>;

    fn get_players(&self) -> Vec<Arc<Mutex<Player>>>;

    fn refresh_players(&mut self) -> Vec<Arc<Mutex<Player>>>;

    fn create_game_id(&mut self);

    fn reset_game_id(&mut self);

    fn set_room_id(&mut self, room_id: i32);

    fn has_players_required(&self) -> bool;

    fn remove_player(&mut self, player: &Arc<Mutex<Player>>);

    fn game_start(&mut self);

    fn game_stop(&mut self);

    fn get_game_fuse_type(&self) -> String;

    fn get_minimum_people_required(&self) -> i32;

    fn get_maximum_people_required(&self) -> i32;

    fn handle_command(
        &mut self,
        player: &Arc<Mutex<Player>>,
        room: &Room,
        item: &Item,
        command: &str,
        args: &[String],
    );

    fn send_to_everyone(&mut self, message: &Arc<dyn MessageComposer + Send + Sync>);
}
