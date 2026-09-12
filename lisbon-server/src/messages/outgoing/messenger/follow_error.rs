//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.FOLLOW_ERROR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FOLLOW_ERROR {
    error_id: i32,
}

impl FOLLOW_ERROR {
    /// Mirrors the `FOLLOW_ERROR(int)` constructor.
    pub fn new(error_id: i32) -> Self {
        Self { error_id }
    }
}

impl MessageComposer for FOLLOW_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.error_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        349 // "E]"
    }
}
