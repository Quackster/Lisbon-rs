//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.badges.AVAILABLE_BADGES`.
use crate::game::badges::badge::Badge;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct AVAILABLE_BADGES {
    badges: Vec<Badge>,
    equipped_badges: Vec<Badge>,
}

impl AVAILABLE_BADGES {
    /// Mirrors the `AVAILABLE_BADGES(List<Badge>, List<Badge>)` constructor.
    pub fn new(badges: Vec<Badge>, equipped_badges: Vec<Badge>) -> Self {
        Self {
            badges,
            equipped_badges,
        }
    }
}

impl MessageComposer for AVAILABLE_BADGES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.badges.len() as i32);

        for badge in &self.badges {
            response.write_string(badge.get_badge_code());
        }

        response.write_int(self.equipped_badges.len() as i32);

        for badge in &self.equipped_badges {
            response.write_int(badge.get_slot_id());
            response.write_string(badge.get_badge_code());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        229 // "Ce"
    }
}
