//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_MARKREAD`.
use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_MARKREAD;

impl MessageEvent for MESSENGER_MARKREAD {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let message_id = reader.read_int();

        MessengerDao::mark_message_read(message_id);

        if let Some(messenger) = player.get_messenger() {
            messenger.get_offline_messages().remove(&message_id);
        }

        Ok(())
    }
}
