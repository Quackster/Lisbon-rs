//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.INITIATEJOINGAME`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::player::player::Player;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::messages::outgoing::games::joinfailed::{JOINFAILED, FailedReason};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct INITIATEJOINGAME;

impl MessageEvent for INITIATEJOINGAME {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let instance_id = reader.read_int();
        let team_id = reader.read_int();

        let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) else {
            return Ok(());
        };
        if trigger.as_game_lobby().is_none() {
            return Ok(());
        }

        let Some(game) = GameManager::get_instance().get_game_by_id(instance_id) else {
            return Ok(());
        };

        if game.get_game_state() != GameState::Waiting {
            return Ok(());
        }

        if player.get_details().get_tickets() < game.get_ticket_cost() {
            player.send(&JOINFAILED::new(FailedReason::TicketsNeeded, None));
            return Ok(());
        }

        if !game.can_switch_team(team_id) {
            player.send(&JOINFAILED::new(FailedReason::TeamsFull, Some("join")));
            return Ok(());
        }

        // If player was initially a spectator, they need to leave
        let existing_game_player = room_user.get_game_player();
        if let Some(existing) = &existing_game_player {
            if existing.lock().is_spectator() {
                game.leave_game(existing);
            }
        }

        // Their game player instance will always be null after leaveGame()
        if room_user.get_game_player().is_none() {
            // The Java `new GamePlayer(player)` needs the owning
            // `Arc<Mutex<Player>>`; it is resolved back via the
            // entity manager.
            let player_id = player.get_details().get_id();
            let Some(player_arc) = room
                .get_entity_manager()
                .get_players()
                .into_iter()
                .find(|p| p.lock().get_details().get_id() == player_id)
            else {
                return Ok(());
            };

            let game_player = Arc::new(Mutex::new(GamePlayer::new(player_arc)));
            game_player.lock().set_game_id(game.get_id());
            game_player.lock().set_team_id(team_id);
            room_user.set_game_player(game_player);
        }

        let Some(game_player) = room_user.get_game_player() else {
            return Ok(());
        };

        // The Java `removeObserver(player)` uses the plain `Player`.
        let player_id = player.get_details().get_id();
        let player_arc = room
            .get_entity_manager()
            .get_players()
            .into_iter()
            .find(|p| p.lock().get_details().get_id() == player_id);

        if let Some(player_arc) = player_arc {
            game.remove_observer(&player_arc);
        }

        game.move_player(&game_player, game_player.lock().get_team_id(), team_id);

        Ok(())
    }
}
