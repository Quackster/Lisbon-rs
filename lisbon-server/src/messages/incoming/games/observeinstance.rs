//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.OBSERVEINSTANCE`.
use crate::game::games::enums::game_state::GameState;
use crate::game::games::game_manager::GameManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::gameinstance::GAMEINSTANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct OBSERVEINSTANCE;

impl MessageEvent for OBSERVEINSTANCE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) else {
            return Ok(());
        };
        let Some(lobby_trigger) = trigger.as_game_lobby() else {
            return Ok(());
        };

        let game_id = reader.read_int();

        let game = GameManager::get_instance().get_game_by_id(game_id);

        if let Some(game) = &game {
            if game.get_game_state() != GameState::Ended {
                player.send(&GAMEINSTANCE::new_game(game.clone()));
                room_user.set_observing_game_id(game_id);

                // The Java `addObserver` takes the plain `Player`;
                // it is resolved back to the Arc via the entity
                // manager.
                let player_id = player.get_details().get_id();
                let player_arc = room
                    .get_entity_manager()
                    .get_players()
                    .into_iter()
                    .find(|p| p.lock().get_details().get_id() == player_id);

                if let Some(player_arc) = player_arc {
                    game.add_observer(player_arc);
                }
                return Ok(());
            }
        }

        let manager = GameManager::get_instance();
        let finished_game =
            manager.get_finished_game_by_id(lobby_trigger.get_game_type(), game_id);

        if let Some(finished_game) = finished_game {
            player.send(&GAMEINSTANCE::new_history(finished_game.clone()));
        }

        Ok(())
    }
}
