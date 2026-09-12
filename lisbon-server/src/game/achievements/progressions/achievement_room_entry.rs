//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementRoomEntry`.

use crate::dao::mysql::room_visits_dao::RoomVisitsDao;
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;

pub struct AchievementRoomEntry;

impl AchievementProgress for AchievementRoomEntry {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let mut progress = RoomVisitsDao::count_visits(player.get_details().get_id());

        if progress > achievement_info.get_progress_required() {
            progress = achievement_info.get_progress_required();
        }

        if progress != user_achievement.get_progress() {
            user_achievement.set_progress(progress);
            return true;
        }

        false
    }
}
