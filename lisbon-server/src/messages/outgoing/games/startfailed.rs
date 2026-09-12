//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.STARTFAILED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the nested `STARTFAILED.FailedReason` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FailedReason {
    MinimumTeamsRequired,
}

impl FailedReason {
    /// Mirrors `getReasonId()`.
    pub fn get_reason_id(&self) -> i32 {
        match self {
            Self::MinimumTeamsRequired => 8,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct STARTFAILED {
    reason: FailedReason,
    key: Option<String>,
}

impl STARTFAILED {
    /// Mirrors the `STARTFAILED(FailedReason, String)` constructor.
    pub fn new(reason: FailedReason, key: Option<&str>) -> Self {
        Self {
            reason,
            key: key.map(String::from),
        }
    }
}

impl MessageComposer for STARTFAILED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.reason.get_reason_id());

        if let Some(key) = &self.key {
            response.write_string(key.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        242
    }
}
