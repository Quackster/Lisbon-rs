//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.USER_CANCEL_TYPING`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::user::typing_status::TYPING_STATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct USER_CANCEL_TYPING;

impl MessageEvent for USER_CANCEL_TYPING {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room_user.is_typing() {
            return Ok(());
        }

        if let Some(game_player) = room_user.get_game_player() {
            let game_player = game_player.lock();

            if game_player.get_game().is_some() && game_player.is_in_game() {
                return Ok(());
            }
        }

        room_user.stop_chat_bubble_timer();
        room_user.set_typing(false);

        room.send(&TYPING_STATUS::new(
            room_user.get_instance_id(),
            room_user.is_typing(),
        ));

        Ok(())
    }
}
