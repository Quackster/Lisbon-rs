//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.pool.DIVE`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::pool::jumpdata::JUMPDATA;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DIVE;

impl MessageEvent for DIVE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room_user.is_diving() {
            return Ok(());
        }

        let diving_handle = reader.contents().unwrap_or_default();

        room.send(&JUMPDATA::new(room_user.get_instance_id(), &diving_handle));

        Ok(())
    }
}
