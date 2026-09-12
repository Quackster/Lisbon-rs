//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMESTART`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GAMESTART {
    game_length_seconds: i32,
}

impl GAMESTART {
    /// Mirrors the `GAMESTART(int)` constructor.
    pub fn new(game_length_seconds: i32) -> Self {
        Self {
            game_length_seconds,
        }
    }
}

impl MessageComposer for GAMESTART {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.game_length_seconds);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        247 // "Cw"
    }
}
