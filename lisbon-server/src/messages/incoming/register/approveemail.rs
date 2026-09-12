//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.APPROVEEMAIL`.
use crate::game::player::player::Player;
use crate::messages::outgoing::register::email_approved::EMAIL_APPROVED;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct APPROVEEMAIL;

impl MessageEvent for APPROVEEMAIL {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if player.is_logged_in() {
            return Ok(());
        }

        let _email = reader.read_string();

        player.send(&EMAIL_APPROVED);

        Ok(())
    }
}
