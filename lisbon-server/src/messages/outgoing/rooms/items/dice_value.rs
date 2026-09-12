//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.DICE_VALUE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct DICE_VALUE {
    item_id: i32,
    spin: bool,
    random_number: i32,
}

impl DICE_VALUE {
    /// Mirrors the `DICE_VALUE(int, boolean, int)` constructor.
    pub fn new(item_id: i32, spin: bool, random_number: i32) -> Self {
        Self {
            item_id,
            spin,
            random_number,
        }
    }
}

impl MessageComposer for DICE_VALUE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.item_id);

        if !self.spin {
            if self.random_number > 0 {
                response.write(format!(" {}", (self.item_id * 38) + self.random_number));
            } else {
                response.write(format!(" {}", self.item_id * 38));
            }
        }
    }

    fn get_header(&self) -> i16 {
        90 // "AZ"
    }
}
