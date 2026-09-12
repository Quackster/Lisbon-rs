//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.FLAT_LETIN`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FLAT_LETIN;

impl MessageComposer for FLAT_LETIN {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        41 // "@i"
    }
}
