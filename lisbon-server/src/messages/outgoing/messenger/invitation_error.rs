//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.INVITATION_ERROR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INVITATION_ERROR;

impl MessageComposer for INVITATION_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        262
    }
}
