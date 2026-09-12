//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.pool.OPEN_UIMAKOPPI`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct OPEN_UIMAKOPPI;

impl MessageComposer for OPEN_UIMAKOPPI {
    /// Mirrors `compose(NettyResponse)` (an empty acknowledgement).
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        // "A`"
        96
    }
}
