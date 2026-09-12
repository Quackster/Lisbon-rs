//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.NOFLATSFORUSER`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct NOFLATSFORUSER {
    username: String,
}

impl NOFLATSFORUSER {
    /// Mirrors the `NOFLATSFORUSER(String)` constructor.
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

impl MessageComposer for NOFLATSFORUSER {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.username.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        57 // "@y"
    }
}
