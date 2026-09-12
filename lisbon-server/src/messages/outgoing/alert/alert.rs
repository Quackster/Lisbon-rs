//! Mirrors `net.h4bbo.lisbon.messages.outgoing.alert.ALERT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
pub struct ALERT {
    message: String,
}

impl ALERT {
    /// Mirrors the `ALERT(String)` constructor.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl MessageComposer for ALERT {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(&self.message);
    }

    fn get_header(&self) -> i16 {
        139 // "BK"
    }
}
