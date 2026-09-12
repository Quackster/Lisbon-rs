//! Mirrors `net.h4bbo.lisbon.messages.outgoing.register.PASSWORD_APPROVED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct PASSWORD_APPROVED {
    error_code: i32,
}

impl PASSWORD_APPROVED {
    /// Mirrors the `PASSWORD_APPROVED(int)` constructor.
    pub fn new(error_code: i32) -> Self {
        Self { error_code }
    }
}

impl MessageComposer for PASSWORD_APPROVED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.error_code);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        282 // "DZ"
    }
}
