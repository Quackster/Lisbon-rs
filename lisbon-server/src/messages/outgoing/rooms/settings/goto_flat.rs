//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.settings.GOTO_FLAT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GOTO_FLAT {
    room_id: i32,
    room_name: String,
}

impl GOTO_FLAT {
    /// Mirrors the `GOTO_FLAT(int, String)` constructor.
    pub fn new(room_id: i32, room_name: &str) -> Self {
        Self {
            room_id,
            room_name: room_name.to_string(),
        }
    }
}

impl MessageComposer for GOTO_FLAT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_delimeter(self.room_id, '\u{000D}');
        response.write(self.room_name.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        59 // "@{"
    }
}
