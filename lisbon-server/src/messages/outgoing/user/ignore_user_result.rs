//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.IGNORE_USER_RESULT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct IGNORE_USER_RESULT {
    result: i32,
}

impl IGNORE_USER_RESULT {
    /// Mirrors the `IGNORE_USER_RESULT(int)` constructor.
    pub fn new(result: i32) -> Self {
        Self { result }
    }
}

impl MessageComposer for IGNORE_USER_RESULT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.result);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        419
    }
}
