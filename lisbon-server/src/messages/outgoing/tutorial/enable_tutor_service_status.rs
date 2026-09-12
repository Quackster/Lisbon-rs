//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.ENABLE_TUTOR_SERVICE_STATUS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TutorEnableStatus {
    FriendlistFull,
    ServiceDisabled,
    MaxNewbies,
}

impl TutorEnableStatus {
    /// Mirrors `getStateId()`.
    pub fn get_state_id(&self) -> i32 {
        match self {
            TutorEnableStatus::FriendlistFull => 2,
            TutorEnableStatus::ServiceDisabled => 3,
            TutorEnableStatus::MaxNewbies => 4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct ENABLE_TUTOR_SERVICE_STATUS {
    status: TutorEnableStatus,
}

impl ENABLE_TUTOR_SERVICE_STATUS {
    /// Mirrors the `ENABLE_TUTOR_SERVICE_STATUS(TutorEnableStatus)` constructor.
    pub fn new(status: TutorEnableStatus) -> Self {
        Self { status }
    }
}

impl MessageComposer for ENABLE_TUTOR_SERVICE_STATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.status.get_state_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        426
    }
}
