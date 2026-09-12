//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_GETREQUESTS`.
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::friend_requests::FRIEND_REQUESTS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_GETREQUESTS;

impl MessageEvent for MESSENGER_GETREQUESTS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        player.send(&FRIEND_REQUESTS::new(messenger.get_requests()));

        Ok(())
    }
}
