//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementLogin`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;

pub struct AchievementLogin;

impl AchievementProgress for AchievementLogin {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let mut progress = 0;

        if progress > 0 {
            player
                .get_statistic_manager()
                .increment_value(PlayerStatistic::DaysLoggedInRow, progress);
            progress = player
                .get_statistic_manager()
                .get_int_value(PlayerStatistic::DaysLoggedInRow);
        }

        if progress > achievement_info.get_progress_required() {
            progress = achievement_info.get_progress_required();
        }

        if progress > 0 {
            user_achievement.set_progress(progress);
            return true;
        }

        false
    }
}
