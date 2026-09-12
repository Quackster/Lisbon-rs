//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.INVITE_CANCELLED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct INVITE_CANCELLED;

impl MessageComposer for INVITE_CANCELLED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        360
    }
}
