//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.USER_OBJECT`.
use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_OBJECT {
    details: PlayerDetails,
}

impl USER_OBJECT {
    /// Mirrors the `USER_OBJECT(PlayerDetails)` constructor.
    pub fn new(details: PlayerDetails) -> Self {
        Self { details }
    }
}

impl MessageComposer for USER_OBJECT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.details.get_id());
        response.write_string(self.details.get_name());
        response.write_string(self.details.get_figure());
        response.write_string(self.details.get_sex());
        response.write_string(self.details.get_motto());
        response.write_int(self.details.get_tickets());
        response.write_string(self.details.get_pool_figure());
        response.write_int(self.details.get_film());
        response.write_bool(false);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        5 // "@E"
    }
}
