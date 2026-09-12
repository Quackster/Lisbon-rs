//! Mirrors `net.h4bbo.lisbon.dao.mysql.GuideDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::player::guides::guiding_data::GuidingData;

pub struct GuideDao;

impl GuideDao {
    /// Mirrors `getGuidedBy(int)`.
    pub fn get_guided_by(user_id: i32) -> Vec<GuidingData> {
        let mut users = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT id, username, last_online, online_time FROM users_statistics INNER JOIN users ON users.id = users_statistics.user_id WHERE guided_by = {user_id}"
            ),
        ) {
            if let (Some(id), Some(username), Some(last_online), Some(online_time)) = (
                row.i32("id"),
                row.str("username"),
                row.i64("last_online"),
                row.i64("online_time"),
            ) {
                users.push(GuidingData::new(id, &username, last_online, online_time));
            }
        }

        users
    }
}
