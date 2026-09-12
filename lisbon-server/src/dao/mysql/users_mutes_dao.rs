//! Mirrors `net.h4bbo.lisbon.dao.mysql.UsersMutesDao`.

use crate::dao::storage::{RowGetters, Storage};

pub struct UsersMutesDao;

impl UsersMutesDao {
    /// Mirrors `getMutedUsers(int)`.
    pub fn get_muted_users(user_id: i32) -> Vec<i32> {
        let mut users = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT muted_id FROM users_mutes WHERE user_id = {user_id}"))
        {
            if let Some(muted_id) = row.i32("muted_id") {
                users.push(muted_id);
            }
        }

        users
    }

    /// Mirrors `addMuted(int, int)`.
    pub fn add_muted(user_id: i32, muted_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_mutes (user_id, muted_id) VALUES ({user_id}, {muted_id})"
        ));
    }

    /// Mirrors `removeMuted(int, int)`.
    pub fn remove_muted(user_id: i32, muted_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_mutes WHERE user_id = {user_id} AND muted_id = {muted_id}"
        ));
    }
}
