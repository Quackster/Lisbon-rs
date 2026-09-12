//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.SECRET_KEY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SECRET_KEY {
    key: String,
}

impl SECRET_KEY {
    /// Mirrors the `SECRET_KEY(String)` constructor.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

impl MessageComposer for SECRET_KEY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.key.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        1 // "@A"
    }
}
