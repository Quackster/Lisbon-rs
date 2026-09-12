//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.AGE_CHECK`.
use crate::game::player::player::Player;
use crate::messages::outgoing::register::age_check_result::AGE_CHECK_RESULT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct AGE_CHECK;

impl MessageEvent for AGE_CHECK {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&AGE_CHECK_RESULT);

        Ok(())
    }
}
