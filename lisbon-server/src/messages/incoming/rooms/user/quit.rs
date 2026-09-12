//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.QUIT`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct QUIT;

impl MessageEvent for QUIT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        room_user.set_authenticate_teleporter_id(-1);
        room_user.set_authenticate_id(-1);

        room.get_entity_manager().leave_room(&room, player, false);

        Ok(())
    }
}
