//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementHappyHour`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;
use crate::lisbon::Lisbon;

pub struct AchievementHappyHour;

impl AchievementProgress for AchievementHappyHour {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        _player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        if Lisbon::is_happy_hour() {
            user_achievement.set_progress(achievement_info.get_progress_required());
            return true;
        }

        false
    }
}
