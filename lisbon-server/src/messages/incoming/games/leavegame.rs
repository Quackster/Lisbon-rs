//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.LEAVEGAME`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct LEAVEGAME;

impl MessageEvent for LEAVEGAME {
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
        let game = game_player_arc.lock().get_game();

        let Some(game) = game else {
            return Ok(());
        };

        game.leave_game(&game_player_arc);

        Ok(())
    }
}
