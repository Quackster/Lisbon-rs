//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.ROOM_AD`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct ROOM_AD;

impl MessageComposer for ROOM_AD {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        208 // "CP"
    }
}
