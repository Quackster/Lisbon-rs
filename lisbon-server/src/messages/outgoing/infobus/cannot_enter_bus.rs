//! Mirrors `net.h4bbo.lisbon.messages.outgoing.infobus.CANNOT_ENTER_BUS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CANNOT_ENTER_BUS {
    message: String,
}

impl CANNOT_ENTER_BUS {
    /// Mirrors the `CANNOT_ENTER_BUS(String)` constructor.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl MessageComposer for CANNOT_ENTER_BUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.message.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        81
    }
}
