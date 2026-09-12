//! Mirrors `net.h4bbo.lisbon.messages.outgoing.alert.NO_USER_FOUND`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct NO_USER_FOUND {
    username: String,
}

impl NO_USER_FOUND {
    /// Mirrors the `NO_USER_FOUND(String)` constructor.
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

impl MessageComposer for NO_USER_FOUND {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.username.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        76 // "AL"
    }
}
