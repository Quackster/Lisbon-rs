//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.GETINTEREST`.
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::room_interest::ROOM_INTEREST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETINTEREST;

impl MessageEvent for GETINTEREST {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&ROOM_INTEREST);

        Ok(())
    }
}
