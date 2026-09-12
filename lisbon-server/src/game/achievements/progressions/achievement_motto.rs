//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementMotto`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;

pub struct AchievementMotto;

impl AchievementProgress for AchievementMotto {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        _player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        user_achievement.set_progress(achievement_info.get_progress_required());
        true
    }
}
