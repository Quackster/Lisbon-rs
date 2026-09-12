//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.MESSENGER_MSG`.
use crate::game::messenger::messenger_message::MessengerMessage;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct MESSENGER_MSG {
    message: MessengerMessage,
}

impl MESSENGER_MSG {
    /// Mirrors the `MESSENGER_MSG(MessengerMessage)` constructor.
    pub fn new(message: MessengerMessage) -> Self {
        Self { message }
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> &MessengerMessage {
        &self.message
    }
}

impl MessageComposer for MESSENGER_MSG {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.message.get_from_id());
        response.write_string(self.message.get_message());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        134 // "BF"
    }
}
