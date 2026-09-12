//! Mirrors `org.alexdev.http.dao.HomeEditDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::util::date_util::DateUtil;

pub struct HomeEditDao;

impl HomeEditDao {
    /// Mirrors `createSession(int)`.
    pub fn create_session(user_id: i32) {
        let expire = DateUtil::get_current_time_seconds() as i64 + 1800;

        Storage::get_storage().execute(&format!(
            "INSERT INTO homes_edit_sessions (user_id, expire) VALUES ({user_id}, {expire})"
        ));
    }

    /// Mirrors `hasSession(int)`.
    pub fn has_session(user_id: i32) -> bool {
        let now = DateUtil::get_current_time_seconds() as i64;

        for _row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM homes_edit_sessions WHERE user_id = {user_id} AND expire > {now} LIMIT 1"))
        {
            return true;
        }

        false
    }

    /// Mirrors `getSession(int)`.
    pub fn get_session(user_id: i32) -> i64 {
        let mut expire_date = -1;
        let now = DateUtil::get_current_time_seconds() as i64;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM homes_edit_sessions WHERE user_id = {user_id} AND expire > {now} LIMIT 1"))
        {
            if let Some(value) = row.i64("expire") {
                expire_date = value;
            }
        }

        expire_date
    }

    /// Mirrors `delete(int)`.
    pub fn delete(user_id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM homes_edit_sessions WHERE user_id = {user_id}"));
    }
}
