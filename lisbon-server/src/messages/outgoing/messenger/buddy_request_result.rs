//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.BUDDY_REQUEST_RESULT`.
use crate::game::messenger::messenger_error::MessengerError;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct BUDDY_REQUEST_RESULT {
    errors: Vec<MessengerError>,
}

impl BUDDY_REQUEST_RESULT {
    /// Mirrors the `BUDDY_REQUEST_RESULT(List<MessengerError>)` constructor.
    pub fn new(errors: Vec<MessengerError>) -> Self {
        Self { errors }
    }
}

impl MessageComposer for BUDDY_REQUEST_RESULT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.errors.len() as i32);

        for error in &self.errors {
            response.write_string(error.get_causer().unwrap_or(""));
            response.write_int(error.get_error_type().get_error_code());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        315 // "D{"
    }
}
