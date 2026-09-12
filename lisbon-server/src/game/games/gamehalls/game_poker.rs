//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.GamePoker`.

use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::gamehalls::gamehall_game::{GamehallGame, GamehallGameHandle};
use crate::messages::types::MessageComposer;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::room::Room;

/// Mirrors `GamePoker`.
pub struct GamePoker {
    base: GamehallGame,
}

impl GamePoker {
    /// Mirrors the `GamePoker(List<int[]>)` constructor.
    pub fn new(chair_coordinates: Vec<[i32; 2]>) -> Self {
        Self {
            base: GamehallGame::new(chair_coordinates),
        }
    }

    /// Mirrors `gameStart()`.
    pub fn game_start(&mut self) {}

    /// Mirrors `gameStop()`.
    pub fn game_stop(&mut self) {}

    /// Mirrors `joinGame(Player)`.
    pub fn join_game(&mut self, _player: &Arc<Mutex<Player>>) {}

    /// Mirrors `leaveGame(Player)`.
    pub fn leave_game(&mut self, _player: &Arc<Mutex<Player>>) {}

    /// Mirrors `handleCommand(Player, Room, Item, String, String[])`.
    pub fn handle_command(
        &mut self,
        player: &Arc<Mutex<Player>>,
        _room: &Room,
        item: &Item,
        command: &str,
        _args: &[String],
    ) {
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

    /// Mirrors `getMaximumPeopleRequired()`.
    pub fn get_maximum_people_required(&self) -> i32 {
        4
    }

    /// Mirrors `getMinimumPeopleRequired()`.
    pub fn get_minimum_people_required(&self) -> i32 {
        2
    }

    /// Mirrors `getGameFuseType()`.
    pub fn get_game_fuse_type(&self) -> String {
        "Poker".to_string()
    }
}

fn get_trigger(item: &Item) -> Option<&PokerTrigger> {
    item.get_definition()
        .get_interaction_type()
        .and_then(|interaction_type| interaction_type.get_trigger())
        .and_then(|trigger| trigger.downcast_ref::<PokerTrigger>())
}

/// Mirrors the `GamehallGame` upcast for `GamePoker`.
impl GamehallGameHandle for GamePoker {
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
