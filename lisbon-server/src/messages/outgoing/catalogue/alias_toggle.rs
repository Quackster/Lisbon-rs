//! Mirrors `net.h4bbo.lisbon.messages.outgoing.catalogue.ALIAS_TOGGLE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ALIAS_TOGGLE;

impl MessageComposer for ALIAS_TOGGLE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(false);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        297 // "Di"
    }
}
