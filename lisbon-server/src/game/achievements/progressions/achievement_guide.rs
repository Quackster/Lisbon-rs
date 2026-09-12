//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementGuide`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;

pub struct AchievementGuide;

impl AchievementProgress for AchievementGuide {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        _achievement_info: &AchievementInfo,
    ) -> bool {
        let progress = player
            .get_statistic_manager()
            .get_int_value(PlayerStatistic::PlayersGuided);

        if progress >= user_achievement.get_progress() {
            user_achievement.set_progress(progress);
            return true;
        }

        false
    }
}
