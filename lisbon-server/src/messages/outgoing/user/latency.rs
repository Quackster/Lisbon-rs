//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.LATENCY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct LATENCY {
    latency: i32,
}

impl LATENCY {
    /// Mirrors the `LATENCY(int)` constructor.
    pub fn new(latency: i32) -> Self {
        Self { latency }
    }
}

impl MessageComposer for LATENCY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.latency);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        354 // "Eb"
    }
}
