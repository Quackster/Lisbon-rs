//! Mirrors `net.h4bbo.lisbon.messages.incoming.club.SUBSCRIBE_CLUB`.
use crate::game::club::club_subscription::ClubSubscription;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SUBSCRIBE_CLUB;

impl MessageEvent for SUBSCRIBE_CLUB {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        reader.read_string();
        let choice = reader.read_int();

        if ClubSubscription::subscribe_club(player.get_details_mut(), choice) {
            player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));
            player.refresh_club();
        }

        Ok(())
    }
}
