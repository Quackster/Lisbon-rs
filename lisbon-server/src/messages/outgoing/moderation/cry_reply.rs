//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.CRY_REPLY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CRY_REPLY {
    message: String,
}

impl CRY_REPLY {
    /// Mirrors the `CRY_REPLY(String)` constructor.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl MessageComposer for CRY_REPLY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.message.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        274 // "DR"
    }
}
