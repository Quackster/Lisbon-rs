//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.currencies.FILM`.

use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FILM {
    film: i32,
}

impl FILM {
    /// Mirrors the `FILM` constructor.
    pub fn new(details: &PlayerDetails) -> Self {
        Self {
            film: details.get_film(),
        }
    }
}

impl MessageComposer for FILM {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.film);
    }

    fn get_header(&self) -> i16 {
        4
    }
}
