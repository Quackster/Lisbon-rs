//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.TYPING_STATUS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct TYPING_STATUS {
    instance_id: i32,
    typing: bool,
}

impl TYPING_STATUS {
    /// Mirrors the `TYPING_STATUS(int, boolean)` constructor.
    pub fn new(instance_id: i32, typing: bool) -> Self {
        Self {
            instance_id,
            typing,
        }
    }
}

impl MessageComposer for TYPING_STATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);
        response.write_bool(self.typing);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        361 // "Ei"
    }
}
