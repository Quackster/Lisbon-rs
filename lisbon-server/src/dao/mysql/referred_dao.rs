//! Mirrors `net.h4bbo.lisbon.dao.mysql.ReferredDao`.

use crate::dao::storage::{RowGetters, Storage};

pub struct ReferredDao;

impl ReferredDao {
    /// Mirrors `countReferred(int)`.
    pub fn count_referred(id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT COUNT(*) AS referred_count FROM users_referred WHERE user_id = {id}"))
        {
            if let Some(value) = row.i32("referred_count") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `addReferred(int, int)`.
    pub fn add_referred(user_id: i32, referred_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_referred (user_id, referred_id) VALUES ({user_id}, {referred_id})"
        ));
    }
}
