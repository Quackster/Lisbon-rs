//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.GETROOMAD`.
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::room_ad::ROOM_AD;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETROOMAD;

impl MessageEvent for GETROOMAD {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&ROOM_AD);

        Ok(())
    }
}
