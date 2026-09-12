//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_GETMESSAGES`.
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::messenger_msg::MESSENGER_MSG;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_GETMESSAGES;

impl MessageEvent for MESSENGER_GETMESSAGES {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        for offline_message in messenger.get_offline_messages().values() {
            player.send(&MESSENGER_MSG::new(offline_message.clone()));
        }

        Ok(())
    }
}
