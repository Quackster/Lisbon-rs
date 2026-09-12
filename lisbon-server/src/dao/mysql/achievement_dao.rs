//! Mirrors `net.h4bbo.lisbon.dao.mysql.AchievementDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::user::user_achievement::UserAchievement;

pub struct AchievementDao;

impl AchievementDao {
    /// Mirrors `getAchievements()`.
    pub fn get_achievements() -> HashMap<i32, AchievementInfo> {
        let mut achievements_list: HashMap<i32, AchievementInfo> = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM achievements WHERE disabled = 0") {
            if let (Some(id), Some(achievement), Some(level), Some(reward_pixels), Some(progress_needed)) = (
                row.i32("id"),
                row.str("achievement"),
                row.i32("level"),
                row.i32("reward_pixels"),
                row.i32("progress_needed"),
            ) {
                let info = AchievementInfo::new(
                    id,
                    &achievement,
                    level,
                    reward_pixels,
                    progress_needed,
                );

                achievements_list.insert(info.get_id(), info);
            }
        }

        achievements_list
    }

    /// Mirrors `getUserAchievements(int)`.
    pub fn get_user_achievements(user_id: i32) -> Vec<UserAchievement> {
        let mut achievements_list = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users_achievements WHERE user_id = {user_id}"))
        {
            achievements_list.push(UserAchievement::new(
                row.i32("achievement_id").unwrap_or(0),
                row.i32("user_id").unwrap_or(0),
                row.i32("progress").unwrap_or(0),
            ));
        }

        achievements_list
    }

    /// Mirrors `newUserAchievement(int, UserAchievement)`.
    pub fn new_user_achievement(user_id: i32, user_achievement: &UserAchievement) {
        if let Some(achievement_info) = user_achievement.get_achievement_info() {
            Storage::get_storage().execute(&format!(
                "INSERT INTO users_achievements (achievement_id, user_id, progress) VALUES ({}, {user_id}, {})",
                achievement_info.get_id(),
                user_achievement.get_progress()
            ));
        }
    }

    /// Mirrors `saveUserAchievement(int, UserAchievement)`.
    pub fn save_user_achievement(user_id: i32, user_achievement: &UserAchievement) {
        if let Some(achievement_info) = user_achievement.get_achievement_info() {
            Storage::get_storage().execute(&format!(
                "UPDATE users_achievements SET progress = {} WHERE user_id = {user_id} AND achievement_id = {}",
                user_achievement.get_progress(),
                achievement_info.get_id()
            ));
        }
    }
}
