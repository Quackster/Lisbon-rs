//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.AVAILABLE_SETS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct AVAILABLE_SETS {
    set: String,
}

impl AVAILABLE_SETS {
    /// Mirrors the `AVAILABLE_SETS(String)` constructor.
    pub fn new(set: String) -> Self {
        Self { set }
    }
}

impl MessageComposer for AVAILABLE_SETS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(&self.set);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        8 // "@H"
    }
}
