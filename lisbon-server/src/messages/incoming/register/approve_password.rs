//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.APPROVE_PASSWORD`.
use crate::game::player::player::Player;
use crate::messages::outgoing::register::password_approved::PASSWORD_APPROVED;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct APPROVE_PASSWORD;

impl MessageEvent for APPROVE_PASSWORD {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let username = reader.read_string();
        let password = reader.read_string();

        let error_code = if username == password {
            5
        } else if password.len() < 6 {
            1
        } else if password.len() > 10 {
            2
        } else {
            0
        };

        player.send(&PASSWORD_APPROVED::new(error_code));

        Ok(())
    }
}
