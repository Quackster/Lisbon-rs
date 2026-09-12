//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.REMOVE_WALLITEM`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct REMOVE_WALLITEM {
    pub item: Box<Item>,
}

impl REMOVE_WALLITEM {
    pub fn new(item: Box<Item>) -> Self {
        Self { item }
    }
}

impl MessageComposer for REMOVE_WALLITEM {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.item.get_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        84 // "AT"
    }
}
