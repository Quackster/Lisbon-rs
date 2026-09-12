//! Mirrors `net.h4bbo.lisbon.game.triggers.GameLobbyTrigger`.
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_manager::GameManager;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::games::game_player_info::GAMEPLAYERINFO;
use crate::messages::outgoing::games::instance_list::INSTANCELIST;

/// Mirrors the `GameLobbyTrigger` abstract class (the Java
/// `extends GenericTrigger` is the `Trigger` supertrait; the Java
/// `onRoomEntry` / `onRoomLeave` overloads without `firstEntry` are
/// never dispatched and have no Rust equivalent).
pub trait GameLobbyTrigger: Trigger {
    /// Mirrors `createGame(Player, Map<String, Object>)`.
    fn create_game(
        &self,
        _game_creator: &Arc<Mutex<Player>>,
        _game_parameters: &HashMap<String, Box<dyn Any>>,
    );

    /// Mirrors `getGameType()`.
    fn get_game_type(&self) -> GameType;

    /// Mirrors `showPoints(Player, Room)`.
    fn show_points(&self, player: &Arc<Mutex<Player>>, room: &Room) {
        room.send(&GAMEPLAYERINFO::new(self.get_game_type(), vec![player.clone()]));
    }

    /// Mirrors `getInstanceList()`.
    fn get_instance_list(&self) -> INSTANCELIST {
        let manager = GameManager::get_instance();
        let games_by_type = manager.get_games_by_type(self.get_game_type());
        let last_played_games = manager
            .get_last_played_games(self.get_game_type())
            .cloned()
            .unwrap_or_default();

        INSTANCELIST::new(games_by_type, last_played_games)
    }
}
