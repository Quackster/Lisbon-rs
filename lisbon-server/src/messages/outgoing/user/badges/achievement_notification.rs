//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.badges.ACHIEVEMENT_NOTIFICATION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ACHIEVEMENT_NOTIFICATION {
    level: i32,
    badge_code: String,
    badge_remove: Option<String>,
}

impl ACHIEVEMENT_NOTIFICATION {
    /// Mirrors the `ACHIEVEMENT_NOTIFICATION(String, String, int)` constructor.
    pub fn new(badge_code: &str, badge_remove: Option<&str>, level: i32) -> Self {
        Self {
            level,
            badge_code: badge_code.to_string(),
            badge_remove: badge_remove.map(|badge_remove| badge_remove.to_string()),
        }
    }
}

impl MessageComposer for ACHIEVEMENT_NOTIFICATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
        response.write_int(self.level);
        response.write_string(self.badge_code.as_str());
        response.write_string(self.badge_remove.as_deref().unwrap_or(""));
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        437 // "Fu"
    }
}
