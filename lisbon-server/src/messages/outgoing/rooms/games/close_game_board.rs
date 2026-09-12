//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.games.CLOSEGAMEBOARD`.

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct CLOSEGAMEBOARD {
    game_id: String,
    type_: String,
}

impl CLOSEGAMEBOARD {
    /// Mirrors the `CLOSEGAMEBOARD(String, String)` constructor.
    pub fn new(game_id: &str, type_: &str) -> Self {
        Self {
            game_id: game_id.to_string(),
            type_: type_.to_string(),
        }
    }
}

impl MessageComposer for CLOSEGAMEBOARD {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_delimeter(&self.game_id, 9u8);
        response.write_delimeter(&self.type_, 9u8);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        146
    }
}
