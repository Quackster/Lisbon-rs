//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.GAMERESTART`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::game_manager::GameManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::playerrejoined::PLAYERREJOINED;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GAMERESTART;

impl MessageEvent for GAMERESTART {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        if room_user.get_room().is_none() {
            return Ok(());
        }

        let Some(game_player_arc) = room_user.get_game_player() else {
            return Ok(());
        };
        let game_id = game_player_arc.lock().get_game_id();

        let Some(game) = GameManager::get_instance().get_game_by_id(game_id) else {
            return Ok(());
        };

        let clicked_restart = game_player_arc.lock().is_clicked_restart();
        if !game.is_game_finished() || clicked_restart {
            return Ok(());
        }

        // Only allow restart once everyone has clicked they'd like to restart
        game_player_arc.lock().set_clicked_restart(true);
        game.send(&PLAYERREJOINED::new(room_user.get_instance_id()));

        let mut restart_players: Vec<Arc<Mutex<crate::game::games::player::game_player::GamePlayer>>> =
            Vec::new();
        let mut afk_players: Vec<Arc<Mutex<crate::game::games::player::game_player::GamePlayer>>> =
            Vec::new();

        for p in game.get_active_players() {
            if p.lock().is_clicked_restart() {
                restart_players.push(p);
            } else {
                afk_players.push(p);
            }
        }

        if afk_players.is_empty() {
            // Everyone clicked restart
            for p in game.get_active_players() {
                p.lock().set_clicked_restart(false);
            }

            game.restart_game(restart_players);
        }

        Ok(())
    }
}
