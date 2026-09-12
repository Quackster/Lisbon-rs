//! Mirrors `net.h4bbo.lisbon.messages.incoming.catalogue.GET_ALIAS_LIST`.
use crate::game::player::player::Player;
use crate::messages::outgoing::catalogue::alias_toggle::ALIAS_TOGGLE;
use crate::messages::outgoing::catalogue::sprite_list::SPRITE_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_ALIAS_LIST;

impl MessageEvent for GET_ALIAS_LIST {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&SPRITE_LIST);
        player.send(&ALIAS_TOGGLE);

        Ok(())
    }
}
