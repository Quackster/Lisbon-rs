//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.PING`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PING;

impl MessageComposer for PING {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        50 // "@r"
    }
}
