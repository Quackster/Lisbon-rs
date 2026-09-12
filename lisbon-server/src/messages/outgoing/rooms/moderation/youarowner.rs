//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.moderation.YOUAROWNER`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct YOUAROWNER;

impl MessageComposer for YOUAROWNER {
    /// Mirrors `compose(NettyResponse)` (an empty acknowledgement).
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        47 // "@o"
    }
}
