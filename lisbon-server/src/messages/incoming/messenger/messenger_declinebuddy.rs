//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_DECLINEBUDDY`.
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_DECLINEBUDDY;

impl MessageEvent for MESSENGER_DECLINEBUDDY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let decline_all = reader.read_boolean();

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        if decline_all {
            messenger.decline_all_requests();
            return Ok(());
        }

        let amount = reader.read_int();

        for _ in 0..amount {
            let user_id = reader.read_int();

            if !messenger.has_request(user_id) {
                continue;
            }

            let Some(requester) = messenger.get_request(user_id) else {
                continue;
            };

            messenger.decline_request(&requester);
        }

        Ok(())
    }
}
