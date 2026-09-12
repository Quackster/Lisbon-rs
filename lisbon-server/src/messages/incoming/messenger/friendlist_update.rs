//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.FRIENDLIST_UPDATE`.
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::friends_update::FRIENDS_UPDATE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct FRIENDLIST_UPDATE;

impl MessageEvent for FRIENDLIST_UPDATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        player.send(&FRIENDS_UPDATE::new(player, messenger));

        Ok(())
    }
}
