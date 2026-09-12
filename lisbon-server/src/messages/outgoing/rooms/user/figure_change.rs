//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.FIGURE_CHANGE`.
use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FIGURE_CHANGE {
    instance_id: i32,
    details: PlayerDetails,
}

impl FIGURE_CHANGE {
    /// Mirrors the `FIGURE_CHANGE(int, PlayerDetails)` constructor.
    pub fn new(instance_id: i32, details: &PlayerDetails) -> Self {
        Self {
            instance_id,
            details: details.clone(),
        }
    }
}

impl MessageComposer for FIGURE_CHANGE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);
        response.write_string(self.details.get_figure());
        response.write_string(self.details.get_sex());
        response.write_string(self.details.get_motto());
    }

    fn get_header(&self) -> i16 {
        266
    }
}
