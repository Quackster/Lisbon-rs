//! Mirrors `net.h4bbo.lisbon.messages.incoming.welcomingparty.REJECT_TUTOR_INVITATION`.
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REJECT_TUTOR_INVITATION;

impl MessageEvent for REJECT_TUTOR_INVITATION {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, _player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let _user_id = reader.read_string();

        Ok(())
    }
}
