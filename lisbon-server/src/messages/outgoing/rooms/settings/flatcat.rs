//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.settings.FLATCAT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct FLATCAT {
    room_id: i32,
    category_id: i32,
}

impl FLATCAT {
    /// Mirrors the `FLATCAT(int, int)` constructor.
    pub fn new(room_id: i32, category_id: i32) -> Self {
        Self {
            room_id,
            category_id,
        }
    }
}

impl MessageComposer for FLATCAT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.room_id);
        response.write_int(self.category_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        222 // "C^"
    }
}
