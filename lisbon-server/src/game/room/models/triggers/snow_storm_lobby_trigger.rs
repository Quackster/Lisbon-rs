//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.SnowStormLobbyTrigger`.
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::games::game_player_info::GAMEPLAYERINFO;
use crate::messages::outgoing::games::lounge_info::LOUNGEINFO;
use crate::util::config::game_configuration::GameConfiguration;

pub struct SnowStormLobbyTrigger;

impl Trigger for SnowStormLobbyTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        room: &Room,
        _first_entry: bool,
        _custom_args: &[Box<dyn Any>],
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        if !GameConfiguration::get_instance().get_bool(
            format!("{}.create.game.enabled", self.get_game_type().get_name()).as_str(),
        ) {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        player.send(&LOUNGEINFO::new());
        player.send(&GAMEPLAYERINFO::new(
            self.get_game_type(),
            room.get_entity_manager().get_players(),
        ));

        // The Java `showPoints(player, room)` needs the owning
        // `Arc<Mutex<Player>>`; `Entity.as_player()` hands out a
        // borrowed `&Player`, so the lobby `show_points` flow is
        // resolved back to the Arc via the entity manager.
        let player_id = player.get_details().get_id();
        let player_arc = room
            .get_entity_manager()
            .get_players()
            .into_iter()
            .find(|p| p.lock().get_details().get_id() == player_id);

        if let Some(player_arc) = player_arc {
            self.show_points(&player_arc, room);
        }
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(
        &self,
        entity: &dyn Entity,
        room: &Room,
        _custom_args: &[Box<dyn Any>],
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        if let Some(room_user) = player.get_room_user() {
            if room_user.get_observing_game_id() != -1 {
                // The Java `stopObservingGame` uses the owning
                // `Player`; it is resolved back to the Arc via the
                // entity manager.
                let player_id = player.get_details().get_id();
                let player_arc = room
                    .get_entity_manager()
                    .get_players()
                    .into_iter()
                    .find(|p| p.lock().get_details().get_id() == player_id);

                if let Some(player_arc) = player_arc {
                    room_user.stop_observing_game(&player_arc);
                }
            }
        }
    }

}

impl GameLobbyTrigger for SnowStormLobbyTrigger {
    /// Mirrors `createGame(Player, Map<String, Object>)`.
    fn create_game(
        &self,
        game_creator: &Arc<Mutex<Player>>,
        game_parameters: &HashMap<String, Box<dyn Any>>,
    ) {
        let Some(map_id) = game_parameters
            .get("fieldType")
            .and_then(|value| value.downcast_ref::<i32>().copied())
        else {
            return;
        };

        if map_id < 1 || map_id > 7 {
            return;
        }

        let Some(teams) = game_parameters
            .get("numTeams")
            .and_then(|value| value.downcast_ref::<i32>().copied())
        else {
            return;
        };

        if teams < 1 || teams > 4 {
            return;
        }

        let Some(name) = game_parameters
            .get("name")
            .and_then(|value| value.downcast_ref::<String>().cloned())
        else {
            return;
        };

        if name.is_empty() {
            return;
        }

        let Some(length_choice) = game_parameters
            .get("gameLengthChoice")
            .and_then(|value| value.downcast_ref::<i32>().copied())
        else {
            return;
        };

        let game = Game::new_snow_storm(SnowStormGame::new(
            GameManager::get_instance().create_id(),
            map_id,
            name,
            teams,
            game_creator.clone(),
            length_choice,
            false,
        ));

        let game_player = Arc::new(Mutex::new(GamePlayer::new(game_creator.clone())));
        game_player.lock().set_game_id(game.get_id());
        game_player.lock().set_team_id(0);

        game_creator
            .lock()
            .get_room_user()
            .map(|room_user| room_user.set_game_player(Arc::clone(&game_player)));

        game.move_player(&game_player, -1, 0);

        GameManager::get_instance().add_game(game);
    }

    /// Mirrors `getGameType()`.
    fn get_game_type(&self) -> GameType {
        GameType::Snowstorm
    }
}
