//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.USER_BANNED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_BANNED {
    ban_reason: String,
}

impl USER_BANNED {
    /// Mirrors the `USER_BANNED(String)` constructor.
    pub fn new(ban_reason: String) -> Self {
        Self { ban_reason }
    }
}

impl MessageComposer for USER_BANNED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(&self.ban_reason);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        35 // "@c"
    }
}
