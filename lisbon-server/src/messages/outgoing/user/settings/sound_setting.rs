//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.settings.SOUND_SETTING`.
use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SOUND_SETTING {
    details: PlayerDetails,
}

impl SOUND_SETTING {
    /// Mirrors the `SOUND_SETTING(PlayerDetails)` constructor.
    pub fn new(details: PlayerDetails) -> Self {
        Self { details }
    }
}

impl MessageComposer for SOUND_SETTING {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.details.get_sound_setting());
        response.write_int(0);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        308
    }
}
