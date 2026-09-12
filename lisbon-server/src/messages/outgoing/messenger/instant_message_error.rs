//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.INSTANT_MESSAGE_ERROR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INSTANT_MESSAGE_ERROR {
    error_code: i32,
    chat_id: i32,
}

impl INSTANT_MESSAGE_ERROR {
    /// Mirrors the `INSTANT_MESSAGE_ERROR(int, int)` constructor.
    pub fn new(error_code: i32, chat_id: i32) -> Self {
        Self {
            error_code,
            chat_id,
        }
    }
}

impl MessageComposer for INSTANT_MESSAGE_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.error_code);
        response.write_int(self.chat_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        261 // "DE"
    }
}
