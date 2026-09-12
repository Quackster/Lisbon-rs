//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.STARTGAME`.
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_manager::GameManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::games::startfailed::{STARTFAILED, FailedReason};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct STARTGAME;

impl MessageEvent for STARTGAME {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
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

        if game.get_game_state() != GameState::Waiting {
            return Ok(());
        }

        if game.get_game_creator_id() != player.get_details().get_id() {
            return Ok(());
        }

        if !game.can_game_start() {
            if game.get_game_type() == GameType::Snowstorm && game.get_team_amount() == 1 {
                player.send(&ALERT::new(
                    "There needs to be at least two players to start this match",
                ));
            } else {
                player.send(&STARTFAILED::new(FailedReason::MinimumTeamsRequired, None));
            }
            return Ok(());
        }

        game.start_game();

        Ok(())
    }
}
