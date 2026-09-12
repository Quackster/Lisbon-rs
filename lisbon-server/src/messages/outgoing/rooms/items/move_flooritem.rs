//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.MOVE_FLOORITEM`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct MOVE_FLOORITEM {
    pub item: Box<Item>,
}

impl MOVE_FLOORITEM {
    pub fn new(item: Box<Item>) -> Self {
        Self { item }
    }
}

impl MessageComposer for MOVE_FLOORITEM {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        self.item.serialise(response);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        95 // "A_"
    }
}
