//! Mirrors `net.h4bbo.lisbon.messages.outgoing.alert.HOTEL_LOGOUT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the `LogoutReason` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoutReason {
    /// Mirrors `DISCONNECT`.
    Disconnect,
    /// Mirrors `LOGGED_OUT`.
    LoggedOut,
    /// Mirrors `LOGOUT_CONCURRENT`.
    LogoutConcurrent,
    /// Mirrors `LOGOUT_TIMEOUT`.
    LogoutTimeout,
}

impl LogoutReason {
    /// Mirrors `getMsgId()`.
    pub fn get_msg_id(&self) -> i32 {
        match self {
            LogoutReason::Disconnect => -1,
            LogoutReason::LoggedOut => 1,
            LogoutReason::LogoutConcurrent => 2,
            LogoutReason::LogoutTimeout => 3,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct HOTEL_LOGOUT {
    reason: LogoutReason,
}

impl HOTEL_LOGOUT {
    /// Mirrors the `HOTEL_LOGOUT(LogoutReason)` constructor.
    pub fn new(reason: LogoutReason) -> Self {
        Self { reason }
    }
}

impl MessageComposer for HOTEL_LOGOUT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.reason.get_msg_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        287 // "D_"
    }
}
