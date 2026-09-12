//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.INSTANT_MESSAGE_INVITATION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INSTANT_MESSAGE_INVITATION {
    user_id: i32,
    message: String,
}

impl INSTANT_MESSAGE_INVITATION {
    /// Mirrors the `INSTANT_MESSAGE_INVITATION(int, String)` constructor.
    pub fn new(user_id: i32, message: &str) -> Self {
        Self {
            user_id,
            message: message.to_string(),
        }
    }
}

impl MessageComposer for INSTANT_MESSAGE_INVITATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.user_id);
        response.write_string(self.message.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        135
    }
}
