//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.currencies.CREDIT_BALANCE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct CREDIT_BALANCE {
    credits: i32,
}

impl CREDIT_BALANCE {
    /// Mirrors the `CREDIT_BALANCE(int)` constructor.
    pub fn new(credits: i32) -> Self {
        Self { credits }
    }
}

impl MessageComposer for CREDIT_BALANCE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(format!("{}.0", self.credits));
    }

    fn get_header(&self) -> i16 {
        6
    }
}
