//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.GUIDE_FOUND`.

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct GUIDE_FOUND {
    account_id: i32,
}

impl GUIDE_FOUND {
    /// Mirrors the `GUIDE_FOUND(int)` constructor.
    pub fn new(account_id: i32) -> Self {
        Self { account_id }
    }
}

impl MessageComposer for GUIDE_FOUND {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.account_id);
    }

    fn get_header(&self) -> i16 {
        423
    }
}
