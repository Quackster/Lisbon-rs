//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.INVITING_COMPLETED`.

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the nested `INVITING_COMPLETED.InvitationResult` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InvitationResult {
    Success,
    Failure,
}

#[allow(non_camel_case_types)]
pub struct INVITING_COMPLETED {
    result: InvitationResult,
}

impl INVITING_COMPLETED {
    /// Mirrors the `INVITING_COMPLETED(InvitationResult)` constructor.
    pub fn new(result: InvitationResult) -> Self {
        Self { result }
    }
}

impl MessageComposer for INVITING_COMPLETED {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.result == InvitationResult::Success);
    }

    fn get_header(&self) -> i16 {
        357 // "Ee"
    }
}
