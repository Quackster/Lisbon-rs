//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.BROADCAST_TELEPORTER`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct BROADCAST_TELEPORTER {
    item: Item,
    name: String,
    disappear_user: bool,
}

impl BROADCAST_TELEPORTER {
    /// Mirrors the `BROADCAST_TELEPORTER(Item, String, boolean)` constructor.
    pub fn new(item: Item, name: &str, disappear_user: bool) -> Self {
        Self {
            item,
            name: name.to_string(),
            disappear_user,
        }
    }
}

impl MessageComposer for BROADCAST_TELEPORTER {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.item.get_id());
        response.write("/");
        response.write(&self.name);
        response.write("/");
        response.write(self.item.get_definition().get_sprite());
    }

    fn get_header(&self) -> i16 {
        if self.disappear_user {
            // "AY"
            89
        } else {
            // "A\""
            92
        }
    }
}
