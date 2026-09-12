//! Mirrors `net.h4bbo.lisbon.messages.outgoing.openinghours.INFO_HOTEL_CLOSING`.
use std::time::Duration;

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INFO_HOTEL_CLOSING {
    minutes_until: Duration,
}

impl INFO_HOTEL_CLOSING {
    /// Mirrors the `INFO_HOTEL_CLOSING(Duration)` constructor.
    pub fn new(minutes_until: Duration) -> Self {
        Self { minutes_until }
    }
}

impl MessageComposer for INFO_HOTEL_CLOSING {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int((self.minutes_until.as_secs() / 60) as i32);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        291 // "Dc"
    }
}
