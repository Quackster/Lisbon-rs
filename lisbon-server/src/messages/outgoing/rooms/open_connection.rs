//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.OPEN_CONNECTION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct OPEN_CONNECTION;

impl MessageComposer for OPEN_CONNECTION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        19 // "@S"
    }
}
