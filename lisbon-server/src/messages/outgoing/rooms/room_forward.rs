//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ROOMFORWARD`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOMFORWARD {
    pub is_public: bool,
    pub room_id: i32,
}

impl ROOMFORWARD {
    pub fn new(is_public: bool, room_id: i32) -> Self {
        Self {
            is_public,
            room_id,
        }
    }
}

impl MessageComposer for ROOMFORWARD {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.is_public);
        response.write_int(self.room_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        286 // "D^"
    }
}
