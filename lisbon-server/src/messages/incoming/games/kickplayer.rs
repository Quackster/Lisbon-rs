//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.KICKPLAYER`.
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::game_manager::GameManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::createfailed::{CREATEFAILED, FailedReason};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct KICKPLAYER;

impl MessageEvent for KICKPLAYER {
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

        let Some(game_player_arc) = room_user.get_game_player() else {
            return Ok(());
        };
        let game_id = game_player_arc.lock().get_game_id();

        let Some(game) = GameManager::get_instance().get_game_by_id(game_id) else {
            return Ok(());
        };
        if game.get_game_creator_id() != player.get_details().get_id() {
            return Ok(());
        }

        let instance_id = reader.read_int();

        let mut team_player = None;

        'teams: for team in game.get_teams() {
            let active_players = team.lock().get_active_players();
            for p in active_players {
                let p_guard = p.lock();
                let player_guard = p_guard.get_player().lock();
                let Some(p_room_user) = player_guard.get_room_user() else {
                    continue;
                };
                if p_room_user.get_instance_id() == instance_id {
                    team_player = Some(Arc::clone(&p));
                    break 'teams;
                }
            }
        }

        let Some(team_player) = team_player else {
            return Ok(());
        };

        game.leave_game(&team_player);
        team_player
            .lock()
            .get_player()
            .lock()
            .send(&CREATEFAILED::new(FailedReason::Kicked));

        Ok(())
    }
}
