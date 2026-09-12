//! Mirrors `net.h4bbo.lisbon.messages.outgoing.guides.INIT_TUTOR_SERVICE_STATUS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INIT_TUTOR_SERVICE_STATUS {
    status: i32,
}

impl INIT_TUTOR_SERVICE_STATUS {
    /// Mirrors the `INIT_TUTOR_SERVICE_STATUS(int)` constructor.
    pub fn new(status: i32) -> Self {
        Self { status }
    }
}

impl MessageComposer for INIT_TUTOR_SERVICE_STATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.status);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        425 // "Fi"
    }
}
