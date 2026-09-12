//! Mirrors `net.h4bbo.lisbon.dao.mysql.BadgeDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::badges::badge::Badge;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct BadgeDao;

impl BadgeDao {
    /// Mirrors `getRoomBadges()`.
    pub fn get_room_badges() -> HashMap<i32, Vec<String>> {
        let mut badges: HashMap<i32, Vec<String>> = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM rooms_entry_badges") {
            if let (Some(room_id), Some(badge_code)) = (row.i32("room_id"), row.str("badge")) {
                badges.entry(room_id).or_default().push(badge_code);
            }
        }

        badges
    }

    /// Mirrors `deleteRoomBadge(String, String)`.
    pub fn delete_room_badge(room_id: &str, badge_code: &str) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM rooms_entry_badges WHERE room_id = '{}' AND badge = '{}'",
            escape(room_id),
            escape(badge_code)
        ));
    }

    /// Mirrors `createEntryBadge(int, String)`.
    pub fn create_entry_badge(room_id: i32, badge_code: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO rooms_entry_badges (room_id, badge) VALUES ({room_id}, '{}')",
            escape(badge_code)
        ));
    }

    /// Mirrors `updateBadges(Map<Integer, List<String>>)`.
    pub fn update_badges(badges: &HashMap<i32, Vec<String>>) {
        Storage::get_storage().execute("DELETE FROM rooms_entry_badges");

        for (room_id, badges) in badges {
            for badge in badges {
                Self::create_entry_badge(*room_id, badge);
            }
        }
    }

    /// Mirrors `getBadges(int)`.
    pub fn get_badges(user_id: i32) -> Vec<Badge> {
        let mut ranks = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users_badges WHERE user_id = {user_id}"))
        {
            ranks.push(Badge::new(
                row.str("badge").unwrap_or_default().as_str(),
                row.bool("equipped").unwrap_or(false),
                row.i32("slot_id").unwrap_or(0),
            ));
        }

        ranks
    }

    /// Mirrors `newBadge(int, String)`.
    pub fn new_badge(user_id: i32, badge_code: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_badges (user_id, badge) VALUES ({user_id}, '{}')",
            escape(badge_code)
        ));
    }

    /// Mirrors `removeBadge(int, String)`.
    pub fn remove_badge(user_id: i32, badge_code: &str) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_badges WHERE user_id = {user_id} AND badge = '{}'",
            escape(badge_code)
        ));
    }

    /// Mirrors `removeBadge(String)`.
    pub fn remove_badge_by_code(badge_code: &str) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_badges WHERE badge = '{}'",
            escape(badge_code)
        ));
    }

    /// Mirrors `saveBadgeChanges(int, String, boolean, int)`.
    pub fn save_badge_changes(user_id: i32, badge_code: &str, is_equipped: bool, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_badges SET equipped = {}, slot_id = {slot_id} WHERE user_id = {user_id} AND badge = '{}'",
            if is_equipped { 1 } else { 0 },
            escape(badge_code)
        ));
    }

    /// Mirrors `getRankBadges()`.
    pub fn get_rank_badges() -> Vec<String> {
        let mut badges = Vec::new();

        for row in Storage::get_storage().query_all("SELECT badge FROM rank_badges") {
            if let Some(badge) = row.str("badge") {
                badges.push(badge);
            }
        }

        badges
    }
}
