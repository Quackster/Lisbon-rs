//! Mirrors `net.h4bbo.lisbon.messages.incoming.infobus.CHANGEWORLD`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CHANGEWORLD;

impl MessageEvent for CHANGEWORLD {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        // Do not process public room items
        if !room.is_public_room() {
            return Ok(());
        }

        room_user.walk_to(11, 2);

        Ok(())
    }
}
