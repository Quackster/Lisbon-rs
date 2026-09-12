//! Mirrors `net.h4bbo.lisbon.messages.outgoing.recycler.START_RECYCLING_RESULT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct START_RECYCLING_RESULT {
    can_recycle: bool,
}

impl START_RECYCLING_RESULT {
    /// Mirrors the `START_RECYCLING_RESULT(boolean)` constructor.
    pub fn new(can_recycle: bool) -> Self {
        Self { can_recycle }
    }
}

impl MessageComposer for START_RECYCLING_RESULT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.can_recycle {
            response.write_bool(true);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        306
    }
}
