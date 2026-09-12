//! Mirrors `net.h4bbo.lisbon.messages.outgoing.catalogue.NO_CREDITS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct NO_CREDITS;

impl MessageComposer for NO_CREDITS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        68 // "AD"
    }
}
