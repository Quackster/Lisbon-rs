//! Mirrors `net.h4bbo.lisbon.game.achievements.user.UserAchievement`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_manager::AchievementManager;

#[derive(Clone)]
pub struct UserAchievement {
    achievement_id: i32,
    user_id: i32,
    progress: i32,
}

impl UserAchievement {
    /// Mirrors the `UserAchievement(int, int, int)` constructor.
    pub fn new(achievement_id: i32, user_id: i32, progress: i32) -> Self {
        Self {
            achievement_id,
            user_id,
            progress,
        }
    }

    /// Mirrors `getAchievementInfo`.
    pub fn get_achievement_info(&self) -> Option<AchievementInfo> {
        AchievementManager::get_instance().get_achievement(self.achievement_id)
    }

    /// Mirrors `getUserId`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getProgress`.
    pub fn get_progress(&self) -> i32 {
        self.progress
    }

    /// Mirrors `setProgress`.
    pub fn set_progress(&mut self, progress: i32) {
        self.progress = progress
    }
}
