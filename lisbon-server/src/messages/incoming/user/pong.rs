//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.PONG`.
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct PONG;

impl MessageEvent for PONG {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, _reader: &mut NettyRequest) -> Result<(), String> {
        // Nice pong :^)
        player.set_ping_ok(true);
        Ok(())
    }
}
