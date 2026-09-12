//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.TEST_LATENCY`.
use crate::game::player::player::Player;
use crate::messages::outgoing::user::latency::LATENCY;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TEST_LATENCY;

impl MessageEvent for TEST_LATENCY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let latency = reader.read_int();

        player.send(&LATENCY::new(latency));

        Ok(())
    }
}
