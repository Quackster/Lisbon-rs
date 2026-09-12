//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.MODERATOR_ALERT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct MODERATOR_ALERT {
    message: String,
}

impl MODERATOR_ALERT {
    /// Mirrors the `MODERATOR_ALERT(String)` constructor.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl MessageComposer for MODERATOR_ALERT {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(format!("mod_warn/{}", self.message));
    }

    fn get_header(&self) -> i16 {
        33
    }
}
