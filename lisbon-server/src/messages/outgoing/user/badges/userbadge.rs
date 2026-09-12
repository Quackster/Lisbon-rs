//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.badges.USERBADGE`.
use crate::game::badges::badge::Badge;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USERBADGE {
    user_id: i32,
    equipped_badges: Vec<Badge>,
}

impl USERBADGE {
    /// Mirrors the `USERBADGE(int, List<Badge>)` constructor.
    pub fn new(user_id: i32, equipped_badges: Vec<Badge>) -> Self {
        Self {
            user_id,
            equipped_badges,
        }
    }
}

impl MessageComposer for USERBADGE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.user_id);
        response.write_int(self.equipped_badges.len() as i32);

        for badge in &self.equipped_badges {
            response.write_int(badge.get_slot_id());
            response.write_string(badge.get_badge_code());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        228 // "Cd"
    }
}
