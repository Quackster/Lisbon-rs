//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementTags`.

use crate::dao::mysql::tag_dao::TagDao;
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;

pub struct AchievementTags;

impl AchievementProgress for AchievementTags {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let tag_list = TagDao::get_user_tags(player.get_details().get_id());

        let mut progress = tag_list.len() as i32;

        if progress >= 5 {
            progress = achievement_info.get_progress_required();
        }

        if progress >= achievement_info.get_progress_required() {
            user_achievement.set_progress(progress);
            return true;
        }

        false
    }
}
