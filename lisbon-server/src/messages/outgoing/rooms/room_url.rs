//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ROOM_URL`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct ROOM_URL;

impl MessageComposer for ROOM_URL {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string("/client/");
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        166 // "Bf"
    }
}
