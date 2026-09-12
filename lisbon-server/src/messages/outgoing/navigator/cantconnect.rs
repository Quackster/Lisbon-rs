//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.CANTCONNECT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the nested `CANTCONNECT.QueueError` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QueueError {
    Reset,
    Full,
    StaffOnly,
    EventParticipentsOnlyCopy,
    EventParticipentsOnly,
    ClubOnly,
}

impl QueueError {
    /// Mirrors `getReasonType()`.
    pub fn get_reason_type(&self) -> &'static str {
        match self {
            Self::Reset => "queue_reset",
            Self::Full => "queue_full",
            Self::StaffOnly => "na",
            Self::EventParticipentsOnlyCopy => "e2",
            Self::EventParticipentsOnly => "e1",
            Self::ClubOnly => "c",
        }
    }
}

/// Mirrors the nested `CANTCONNECT.ConnectError` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectError {
    RoomFull,
    RoomClosed,
    QueueError,
    Banned,
}

impl ConnectError {
    /// Mirrors `getErrorId()`.
    pub fn get_error_id(&self) -> i32 {
        match self {
            Self::RoomFull => 1,
            Self::RoomClosed => 2,
            Self::QueueError => 3,
            Self::Banned => 4,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CANTCONNECT {
    connect_error: ConnectError,
    queue_error: Option<QueueError>,
}

impl CANTCONNECT {
    /// Mirrors the `CANTCONNECT(ConnectError)` constructor.
    pub fn new_connect_error(connect_error: ConnectError) -> Self {
        Self {
            connect_error,
            queue_error: None,
        }
    }

    /// Mirrors the `CANTCONNECT(QueueError)` constructor.
    pub fn new_queue_error(queue_error: QueueError) -> Self {
        Self {
            connect_error: ConnectError::QueueError,
            queue_error: Some(queue_error),
        }
    }
}

impl MessageComposer for CANTCONNECT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.connect_error.get_error_id());

        if let Some(queue_error) = self.queue_error {
            response.write_string(queue_error.get_reason_type());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        224 // "C`"
    }
}
