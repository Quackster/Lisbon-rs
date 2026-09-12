//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.INVITATION_SENT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct INVITATION_SENT;

impl MessageComposer for INVITATION_SENT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        421 // "Fe"
    }
}
