//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.PLAYERREJOINED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct PLAYERREJOINED {
    instance_id: i32,
}

impl PLAYERREJOINED {
    /// Mirrors the `PLAYERREJOINED(int)` constructor.
    pub fn new(instance_id: i32) -> Self {
        Self { instance_id }
    }
}

impl MessageComposer for PLAYERREJOINED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        245 // "Cu"
    }
}
