//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementGraduate`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;
use crate::util::config::game_configuration::GameConfiguration;

pub struct AchievementGraduate;

impl AchievementProgress for AchievementGraduate {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        _player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        if !GameConfiguration::get_instance().get_bool("tutorial.enabled") {
            return false;
        }

        user_achievement.set_progress(achievement_info.get_progress_required());
        true
    }
}
