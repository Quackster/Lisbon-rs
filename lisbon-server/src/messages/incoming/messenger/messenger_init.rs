//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGERINIT`.
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::messenger_init::MESSENGER_INIT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGERINIT;

impl MessageEvent for MESSENGERINIT {
    /// Mirrors `handle(Player, NettyRequest)` (the Java
    /// `MessengerManager.getMessengerData(id)` resolves to the player's
    /// own messenger here; the player is always online in the handler).
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(messenger) = player.get_messenger().cloned() else {
            return Ok(());
        };

        player.send(&MESSENGER_INIT::new(player, &messenger));

        Ok(())
    }
}
