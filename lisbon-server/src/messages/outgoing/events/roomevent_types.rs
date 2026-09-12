//! Mirrors `net.h4bbo.lisbon.messages.outgoing.events.ROOMEVENT_TYPES`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOMEVENT_TYPES {
    count: i32,
}

impl ROOMEVENT_TYPES {
    /// Mirrors the `ROOMEVENT_TYPES(int)` constructor.
    pub fn new(count: i32) -> Self {
        Self { count }
    }
}

impl MessageComposer for ROOMEVENT_TYPES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.count);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        368
    }
}
