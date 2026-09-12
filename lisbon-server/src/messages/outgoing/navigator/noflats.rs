//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.NOFLATS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct NOFLATS;

impl MessageComposer for NOFLATS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        58 // "@z"
    }
}
