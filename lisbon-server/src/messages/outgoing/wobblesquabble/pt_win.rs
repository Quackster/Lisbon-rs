//! Mirrors `net.h4bbo.lisbon.messages.outgoing.wobblesquabble.PT_WIN`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PT_WIN {
    // The Java field is named `loser` but is assigned from the `winner`
    // constructor argument (mirrored as-is).
    loser: i32,
}

impl PT_WIN {
    /// Mirrors the `PT_WIN(int)` constructor.
    pub fn new(winner: i32) -> Self {
        Self { loser: winner }
    }
}

impl MessageComposer for PT_WIN {
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.loser)
    }

    fn get_header(&self) -> i16 {
        119
    }
}
