//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementEmailVerification`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;

pub struct AchievementEmailVerification;

impl AchievementProgress for AchievementEmailVerification {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        _player: &Player,
        _user_achievement: &mut UserAchievement,
        _achievement_info: &AchievementInfo,
    ) -> bool {
        false
    }
}
