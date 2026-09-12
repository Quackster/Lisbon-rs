//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.WATCHGAME`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::enums::game_state::GameState;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::gameinstance::GAMEINSTANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct WATCHGAME;

impl MessageEvent for WATCHGAME {
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
        if trigger.as_game_lobby().is_none() {
            return Ok(());
        }

        if room_user.get_game_player().is_some() {
            return Ok(());
        }

        let game_id = reader.read_int();

        let Some(game) = GameManager::get_instance().get_game_by_id(game_id) else {
            return Ok(());
        };

        // The Java `new GamePlayer(player)` needs the owning
        // `Arc<Mutex<Player>>`; it is resolved back via the entity
        // manager.
        let player_id = player.get_details().get_id();
        let Some(player_arc) = room
            .get_entity_manager()
            .get_players()
            .into_iter()
            .find(|p| p.lock().get_details().get_id() == player_id)
        else {
            return Ok(());
        };

        let game_player = Arc::new(Mutex::new(GamePlayer::new(Arc::clone(&player_arc))));
        game_player.lock().set_game_id(game_id);
        game_player.lock().set_spectator(true);

        room_user.set_game_player(game_player.clone());

        game.add_spectator(game_player.clone());

        if game.get_game_state() == GameState::Started {
            game.send_spectator_to_arena(&game_player);
        } else {
            player.send(&GAMEINSTANCE::new_game(game.clone()));
            game.send_observers(&GAMEINSTANCE::new_game(game.clone()));
        }

        Ok(())
    }
}
