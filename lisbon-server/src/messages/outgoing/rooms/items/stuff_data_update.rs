//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.STUFFDATAUPDATE`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct STUFFDATAUPDATE {
    pub item: Box<Item>,
}

impl STUFFDATAUPDATE {
    pub fn new(item: Box<Item>) -> Self {
        Self { item }
    }
}

impl MessageComposer for STUFFDATAUPDATE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.item.has_behaviour(ItemBehaviour::WallItem) {
            self.item.serialise(response);
        } else {
            response.write_string(self.item.get_id());
            response.write_string(self.item.get_custom_data());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        if self.item.has_behaviour(ItemBehaviour::WallItem) {
            // "AU"
            85
        } else {
            // "AX"
            88
        }
    }
}
