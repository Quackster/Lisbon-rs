//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.LOGIN`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct LOGIN;

impl MessageComposer for LOGIN {
    /// Mirrors `compose(NettyResponse)` (empty body in the Java source).
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        3 // "@C"
    }
}
