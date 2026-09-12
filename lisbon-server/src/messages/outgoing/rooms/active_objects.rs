//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ACTIVE_OBJECTS`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ACTIVE_OBJECTS {
    items: Vec<Item>,
}

impl ACTIVE_OBJECTS {
    /// Mirrors the `ACTIVE_OBJECTS(List<Item>)` constructor.
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

impl MessageComposer for ACTIVE_OBJECTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.items.len() as i32);

        for item in &self.items {
            item.serialise(response);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        32 // "@`"
    }
}
