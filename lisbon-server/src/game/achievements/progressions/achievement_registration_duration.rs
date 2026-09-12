//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementRegistrationDuration`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::util::date_util::DateUtil;

pub struct AchievementRegistrationDuration;

impl AchievementProgress for AchievementRegistrationDuration {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let days_since_joined =
            (DateUtil::get_current_time_seconds() as i64 - player.get_details().get_join_date()) / 86_400;

        if days_since_joined >= achievement_info.get_progress_required() as i64 {
            user_achievement.set_progress(achievement_info.get_progress_required());
            return true;
        }

        false
    }
}
