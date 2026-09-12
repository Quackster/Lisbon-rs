//! Mirrors `net.h4bbo.lisbon.messages.outgoing.inventory.INVENTORY`.
use std::collections::HashMap;

use crate::game::inventory::inventory::Inventory;
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INVENTORY {
    inventory: Inventory,
    casts: HashMap<i32, Item>,
}

impl INVENTORY {
    /// Mirrors the `INVENTORY(Inventory, Map<Integer, Item>)` constructor.
    pub fn new(inventory: Inventory, casts: HashMap<i32, Item>) -> Self {
        Self { inventory, casts }
    }
}

impl MessageComposer for INVENTORY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for (slot, item) in &self.casts {
            Inventory::serialise(response, item, *slot);
        }

        response.write('\r');
        response.write_int(self.inventory.get_items().len() as i32);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        140 // "BL"
    }
}
