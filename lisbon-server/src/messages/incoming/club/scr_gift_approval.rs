//! Mirrors `net.h4bbo.lisbon.messages.incoming.club.SCR_GIFT_APPROVAL`.
use crate::game::club::club_subscription::ClubSubscription;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SCR_GIFT_APPROVAL;

impl MessageEvent for SCR_GIFT_APPROVAL {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if ClubSubscription::is_gift_due(player) {
            // The Java `SQLException` is carried by the DAO calls.
            ClubSubscription::try_next_gift(player);
        }

        Ok(())
    }
}
