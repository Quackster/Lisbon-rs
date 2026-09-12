//! Mirrors `net.h4bbo.lisbon.messages.outgoing.club.CLUB_GIFT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CLUB_GIFT {
    gift_count: i32,
}

impl CLUB_GIFT {
    /// Mirrors the `CLUB_GIFT(int)` constructor.
    pub fn new(gift_count: i32) -> Self {
        Self { gift_count }
    }
}

impl MessageComposer for CLUB_GIFT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.gift_count);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        280
    }
}
