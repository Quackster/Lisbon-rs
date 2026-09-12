//! Mirrors `net.h4bbo.lisbon.game.achievements.AchievementProgress`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;

/// Mirrors the `AchievementProgress` interface.
pub trait AchievementProgress {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool;
}
