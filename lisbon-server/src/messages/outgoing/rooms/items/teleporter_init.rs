//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.TELEPORTER_INIT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct TELEPORTER_INIT {
    teleporter_id: i32,
    room_id: i32,
}

impl TELEPORTER_INIT {
    /// Mirrors the `TELEPORTER_INIT(int, int)` constructor.
    pub fn new(teleporter_id: i32, room_id: i32) -> Self {
        Self {
            teleporter_id,
            room_id,
        }
    }
}

impl MessageComposer for TELEPORTER_INIT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.teleporter_id);
        response.write_int(self.room_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        62 // "@~"
    }
}
