//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMEDELETED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GAMEDELETED {
    game_id: i32,
}

impl GAMEDELETED {
    /// Mirrors the `GAMEDELETED(int)` constructor.
    pub fn new(game_id: i32) -> Self {
        Self { game_id }
    }
}

impl MessageComposer for GAMEDELETED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.game_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        237 // "Cm"
    }
}
