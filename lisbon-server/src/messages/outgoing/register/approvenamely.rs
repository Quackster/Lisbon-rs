//! Mirrors `net.h4bbo.lisbon.messages.outgoing.register.APPROVENAMERELY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct APPROVENAMERELY {
    name_check_code: i32,
}

impl APPROVENAMERELY {
    /// Mirrors the `APPROVENAMERELY(int)` constructor.
    pub fn new(name_check_code: i32) -> Self {
        Self { name_check_code }
    }
}

impl MessageComposer for APPROVENAMERELY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.name_check_code);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        36
    }
}
