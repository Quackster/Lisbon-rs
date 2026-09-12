//! Mirrors `net.h4bbo.lisbon.game.achievements.progressions.AchievementHabboClub`.

use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_progress::AchievementProgress;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::club::club_subscription::ClubSubscription;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;

pub struct AchievementHabboClub;

impl AchievementProgress for AchievementHabboClub {
    /// Mirrors `tryProgress(Player, UserAchievement, AchievementInfo)`.
    fn try_progress(
        &self,
        player: &Player,
        user_achievement: &mut UserAchievement,
        achievement_info: &AchievementInfo,
    ) -> bool {
        let mut can_progress = false;

        if achievement_info.get_level() == 1 {
            can_progress = player.get_details().has_club_subscription();
        }

        if achievement_info.get_level() == 2 {
            can_progress = ClubSubscription::has_gold_club_subscription(player);
        }

        if achievement_info.get_level() == 3 {
            can_progress = ClubSubscription::has_platinum_club_subscription(player);
        }

        if can_progress {
            user_achievement.set_progress(achievement_info.get_progress_required());
            return true;
        }

        false
    }
}
