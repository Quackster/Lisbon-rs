//! Mirrors `net.h4bbo.lisbon.messages.outgoing.register.EMAIL_APPROVED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct EMAIL_APPROVED;

impl MessageComposer for EMAIL_APPROVED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        271
    }
}
