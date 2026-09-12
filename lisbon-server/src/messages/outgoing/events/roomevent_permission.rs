//! Mirrors `net.h4bbo.lisbon.messages.outgoing.events.ROOMEVENT_PERMISSION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOMEVENT_PERMISSION {
    can_create_event: bool,
}

impl ROOMEVENT_PERMISSION {
    /// Mirrors the `ROOMEVENT_PERMISSION(boolean)` constructor.
    pub fn new(can_create_event: bool) -> Self {
        Self { can_create_event }
    }
}

impl MessageComposer for ROOMEVENT_PERMISSION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.can_create_event);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        367 // "Eo"
    }
}
