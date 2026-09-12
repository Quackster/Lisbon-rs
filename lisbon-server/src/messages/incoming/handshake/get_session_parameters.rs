//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.GET_SESSION_PARAMETERS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::handshake::session_parameters::SESSION_PARAMETERS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_SESSION_PARAMETERS;

impl MessageEvent for GET_SESSION_PARAMETERS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if player.is_logged_in() {
            return Ok(());
        }

        player.send(&SESSION_PARAMETERS::new(player.get_details().clone()));

        Ok(())
    }
}
