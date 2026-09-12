//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.CREATEFAILED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the nested `CREATEFAILED.FailedReason` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FailedReason {
    Kicked,
    TicketsNeeded,
}

impl FailedReason {
    /// Mirrors `getReasonId()`.
    pub fn get_reason_id(&self) -> i32 {
        match self {
            Self::Kicked => 6,
            Self::TicketsNeeded => 2,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CREATEFAILED {
    failed_reason: FailedReason,
}

impl CREATEFAILED {
    /// Mirrors the `CREATEFAILED(FailedReason)` constructor.
    pub fn new(failed_reason: FailedReason) -> Self {
        Self { failed_reason }
    }
}

impl MessageComposer for CREATEFAILED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.failed_reason.get_reason_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        236 // "Cl"
    }
}
