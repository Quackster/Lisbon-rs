//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.GET_CREDITS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_CREDITS;

impl MessageEvent for GET_CREDITS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));

        Ok(())
    }
}
