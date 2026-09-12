//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementGamePlayed`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;

pub struct AchievementGamePlayed;

impl AchievementProgress for AchievementGamePlayed {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let progress = player
            .get_statistic_manager()
            .get_int_value(PlayerStatistic::BattleballGamesWon)
            + player
                .get_statistic_manager()
                .get_int_value(PlayerStatistic::SnowstormGamesWon);

        if progress >= user_achievement.get_progress() {
            let progress = if progress > achievement_info.get_progress_required() {
                achievement_info.get_progress_required()
            } else {
                progress
            };

            user_achievement.set_progress(progress);
            return true;
        }

        false
    }
}
