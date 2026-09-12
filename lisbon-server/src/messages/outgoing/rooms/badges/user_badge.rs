//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.badges.USER_BADGE`.
use crate::game::player::player_details::PlayerDetails;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_BADGE {
    instance_id: i32,
    current_badge: Option<String>,
    show_badge: bool,
}

impl USER_BADGE {
    /// Mirrors the `USER_BADGE(int, PlayerDetails)` constructor.
    pub fn new(instance_id: i32, _player_details: &PlayerDetails) -> Self {
        Self {
            instance_id,
            current_badge: None,
            show_badge: false,
        }
    }
}

impl MessageComposer for USER_BADGE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);

        if self.show_badge {
            response.write_string(self.current_badge.as_deref().unwrap_or(""));
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        228 // "Cd"
    }
}
