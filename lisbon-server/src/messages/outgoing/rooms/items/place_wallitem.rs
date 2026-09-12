//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.PLACE_WALLITEM`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct PLACE_WALLITEM {
    pub item: Box<Item>,
}

impl PLACE_WALLITEM {
    pub fn new(item: Box<Item>) -> Self {
        Self { item }
    }
}

impl MessageComposer for PLACE_WALLITEM {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        self.item.serialise(response);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        83 // "AS"
    }
}
