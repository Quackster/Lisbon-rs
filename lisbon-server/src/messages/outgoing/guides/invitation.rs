//! Mirrors `net.h4bbo.lisbon.messages.outgoing.guides.INVITATION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INVITATION {
    user_id: i32,
    username: String,
}

impl INVITATION {
    /// Mirrors the `INVITATION(Integer, String)` constructor.
    pub fn new(user_id: i32, username: &str) -> Self {
        Self {
            user_id,
            username: username.to_string(),
        }
    }
}

impl MessageComposer for INVITATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.user_id);
        response.write_string(self.username.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        355 // "Ec"
    }
}
