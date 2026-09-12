//! Mirrors `net.h4bbo.lisbon.messages.outgoing.infobus.BUS_DOOR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct BUS_DOOR {
    status: bool,
}

impl BUS_DOOR {
    /// Mirrors the `BUS_DOOR(boolean)` constructor.
    pub fn new(status: bool) -> Self {
        Self { status }
    }
}

impl MessageComposer for BUS_DOOR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.status);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        503
    }
}
