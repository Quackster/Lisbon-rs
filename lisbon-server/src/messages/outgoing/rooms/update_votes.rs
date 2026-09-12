//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.UPDATE_VOTES`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct UPDATE_VOTES {
    pub rating: i32,
}

impl UPDATE_VOTES {
    pub fn new(rating: i32) -> Self {
        Self { rating }
    }
}

impl MessageComposer for UPDATE_VOTES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.rating);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        345
    }
}
