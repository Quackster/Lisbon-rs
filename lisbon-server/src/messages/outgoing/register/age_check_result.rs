//! Mirrors `net.h4bbo.lisbon.messages.outgoing.register.AGE_CHECK_RESULT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct AGE_CHECK_RESULT;

impl MessageComposer for AGE_CHECK_RESULT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(true);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        164
    }
}
