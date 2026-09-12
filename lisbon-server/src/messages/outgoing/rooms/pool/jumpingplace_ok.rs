//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.pool.JUMPINGPLACE_OK`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct JUMPINGPLACE_OK;

impl MessageComposer for JUMPINGPLACE_OK {
    /// Mirrors `compose(NettyResponse)` (an empty acknowledgement).
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        // "A}"
        125
    }
}
