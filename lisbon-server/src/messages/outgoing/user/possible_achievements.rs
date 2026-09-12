//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.POSSIBLE_ACHIEVEMENTS`.
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct POSSIBLE_ACHIEVEMENTS {
    possible_achievements: Vec<AchievementInfo>,
}

impl POSSIBLE_ACHIEVEMENTS {
    /// Mirrors the `POSSIBLE_ACHIEVEMENTS(List<AchievementInfo>)` constructor.
    pub fn new(possible_achievements: Vec<AchievementInfo>) -> Self {
        Self {
            possible_achievements,
        }
    }
}

impl MessageComposer for POSSIBLE_ACHIEVEMENTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.possible_achievements.len() as i32);

        for achievement in &self.possible_achievements {
            response.write_int(achievement.get_id());
            response.write_int(achievement.get_level());

            if achievement.get_name() == "GL" {
                response.write_string(format!(
                    "{}{}",
                    achievement.get_name(),
                    StringUtil::to_alphabetic(achievement.get_level())
                ));
            } else {
                response.write_string(format!(
                    "{}{}",
                    achievement.get_name(),
                    achievement.get_level()
                ));
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        436
    }
}
