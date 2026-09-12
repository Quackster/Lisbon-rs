//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.MESSENGER_ERROR`.
use crate::game::messenger::messenger_error::MessengerError;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct MESSENGER_ERROR {
    client_message_id: i32,
    error: MessengerError,
}

impl MESSENGER_ERROR {
    /// Mirrors the `MESSENGER_ERROR(MessengerError)` constructor.
    pub fn new(error: MessengerError) -> Self {
        Self {
            client_message_id: 0,
            error,
        }
    }
}

impl MessageComposer for MESSENGER_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.client_message_id);
        response.write_int(self.error.get_error_type().get_error_code());

        if let Some(error_reason) = self.error.get_error_reason() {
            response.write_int(error_reason.get_reason_code());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        260 // "DD"
    }
}
