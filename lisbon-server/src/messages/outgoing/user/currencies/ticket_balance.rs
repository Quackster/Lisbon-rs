//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.currencies.TICKET_BALANCE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct TICKET_BALANCE {
    tickets: i32,
}

impl TICKET_BALANCE {
    /// Mirrors the `TICKET_BALANCE(int)` constructor.
    pub fn new(tickets: i32) -> Self {
        Self { tickets }
    }
}

impl MessageComposer for TICKET_BALANCE {
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.tickets)
    }

    fn get_header(&self) -> i16 {
        // "A|"
        124
    }
}
