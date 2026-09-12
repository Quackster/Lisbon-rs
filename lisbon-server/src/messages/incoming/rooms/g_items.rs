//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.G_ITEMS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::items_message::ITEMS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_ITEMS;

impl MessageEvent for G_ITEMS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        player.send(&ITEMS::new(&room));

        Ok(())
    }
}
