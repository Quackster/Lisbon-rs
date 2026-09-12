//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.LOGOUT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct LOGOUT {
    instance_id: i32,
}

impl LOGOUT {
    /// Mirrors the `LOGOUT(int)` constructor.
    pub fn new(instance_id: i32) -> Self {
        Self { instance_id }
    }
}

impl MessageComposer for LOGOUT {
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.instance_id)
    }

    fn get_header(&self) -> i16 {
        // "@]"
        29
    }
}
