//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ROOM_READY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOM_READY {
    room_id: i32,
    model: String,
}

impl ROOM_READY {
    /// Mirrors the `ROOM_READY(int, String)` constructor.
    pub fn new(room_id: i32, model: &str) -> Self {
        Self {
            room_id,
            model: model.to_string(),
        }
    }
}

impl MessageComposer for ROOM_READY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.model.as_str());
        response.write_string(" ");
        response.write_int(self.room_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        69 // "AE"
    }
}
