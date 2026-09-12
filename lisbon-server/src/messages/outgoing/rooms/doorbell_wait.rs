//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.DOORBELL_WAIT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct DOORBELL_WAIT {
    username: Option<String>,
}

impl DOORBELL_WAIT {
    /// Mirrors the no-arg `DOORBELL_WAIT()` constructor.
    pub fn new() -> Self {
        Self { username: None }
    }

    /// Mirrors the `DOORBELL_WAIT(String)` constructor.
    pub fn with_username(username: &str) -> Self {
        Self {
            username: Some(username.to_string()),
        }
    }
}

impl MessageComposer for DOORBELL_WAIT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if let Some(username) = &self.username {
            response.write(username.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        91 // "A["
    }
}
