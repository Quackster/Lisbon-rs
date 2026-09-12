//! Mirrors `net.h4bbo.lisbon.game.achievements.AchievementManager`.

use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::achievement_dao::AchievementDao;
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::util::string_util::StringUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<AchievementManager>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct AchievementManager {
    achievements: HashMap<i32, AchievementInfo>,
}

impl AchievementManager {
    fn new() -> Self {
        Self {
            achievements: AchievementDao::get_achievements(),
        }
    }

    /// Mirrors `tryProgress(AchievementType, Player)`.
    pub fn try_progress(&self, achievement_type: &AchievementType, player: &Player) {
        let Some(mut user_achievement) =
            player.get_achievement_manager().locate_achievement(achievement_type)
        else {
            return;
        };

        let Some(info) = user_achievement.get_achievement_info() else {
            return;
        };

        if achievement_type
            .get_progressor()
            .try_progress(player, &mut user_achievement, &info)
        {
            AchievementDao::save_user_achievement(player.get_details().get_id(), &user_achievement);
        }

        if user_achievement.get_progress() != info.get_progress_required() {
            return;
        }

        let mut badge_code = format!("{}{}", info.get_name(), info.get_level());

        if info.get_name() == "GL" {
            badge_code =
                format!("{}{}", info.get_name(), StringUtil::to_alphabetic(info.get_level()));
        }

        if player.get_badge_manager().has_badge(&badge_code) {
            return;
        }

        let previous_achievement = self.locate_achievement(achievement_type, info.get_level() - 1);

        if let Some(previous_achievement) = previous_achievement {
            let mut badge_remove_code =
                Some(format!("{}{}", previous_achievement.get_name(), previous_achievement.get_level()));

            if !achievement_type.has_remove_previous_achievement() {
                badge_remove_code = None;
            }

            player
                .get_badge_manager()
                .try_add_badge(&badge_code, badge_remove_code.as_deref(), 1);
        } else {
            player
                .get_badge_manager()
                .try_add_badge(&badge_code, None, info.get_level());
        }
    }

    /// Mirrors `locateAchievement(AchievementType, int)`.
    pub fn locate_achievement(
        &self,
        achievement_type: &AchievementType,
        next_level: i32,
    ) -> Option<AchievementInfo> {
        for achievement_info in self.achievements.values() {
            if achievement_info.get_name() == achievement_type.get_name()
                && achievement_info.get_level() == next_level
            {
                return Some(achievement_info.clone());
            }
        }

        None
    }

    /// Mirrors `getAchievement(int)`.
    pub fn get_achievement(&self, achievement_id: i32) -> Option<AchievementInfo> {
        self.achievements.get(&achievement_id).cloned()
    }

    /// Mirrors `getAchievements()`.
    pub fn get_achievements(&self) -> HashMap<i32, AchievementInfo> {
        self.achievements.clone()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<AchievementManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
