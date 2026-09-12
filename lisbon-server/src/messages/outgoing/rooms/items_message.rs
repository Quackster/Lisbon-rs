//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ITEMS`.
use crate::game::item::item::Item;
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ITEMS {
    items: Vec<Item>,
}

impl ITEMS {
    /// Mirrors the `ITEMS(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            items: room.get_item_manager().get_wall_items(),
        }
    }
}

impl MessageComposer for ITEMS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for item in &self.items {
            item.serialise(response);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        45 // "@m"
    }
}
