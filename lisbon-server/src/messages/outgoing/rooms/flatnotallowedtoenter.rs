//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.FLATNOTALLOWEDTOENTER`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FLATNOTALLOWEDTOENTER;

impl MessageComposer for FLATNOTALLOWEDTOENTER {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        131 // "BC"
    }
}
