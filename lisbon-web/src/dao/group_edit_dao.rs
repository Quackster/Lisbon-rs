//! Mirrors `org.alexdev.http.dao.GroupEditDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::util::date_util::DateUtil;

pub struct GroupEditDao;

impl GroupEditDao {
    /// Mirrors `createSession(int, int)`.
    pub fn create_session(user_id: i32, group_id: i32) {
        let expire = DateUtil::get_current_time_seconds() as i64 + 1800;

        Storage::get_storage().execute(&format!(
            "INSERT INTO groups_edit_sessions (user_id, group_id, expire) VALUES ({user_id}, {group_id}, {expire})"
        ));
    }

    /// Mirrors `hasSession(int, int)`.
    pub fn has_session(user_id: i32, group_id: i32) -> bool {
        let now = DateUtil::get_current_time_seconds() as i64;

        for _row in Storage::get_storage().query_all(
            &format!("SELECT * FROM groups_edit_sessions WHERE user_id = {user_id} AND group_id = {group_id} AND expire > {now} LIMIT 1"),
        ) {
            return true;
        }

        false
    }

    /// Mirrors `getSession(int, int)`.
    pub fn get_session(user_id: i32, group_id: i32) -> i64 {
        let mut expire_date = -1;
        let now = DateUtil::get_current_time_seconds() as i64;

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM groups_edit_sessions WHERE user_id = {user_id} AND group_id = {group_id} AND expire > {now} LIMIT 1"),
        ) {
            if let Some(value) = row.i64("expire") {
                expire_date = value;
            }
        }

        expire_date
    }

    /// Mirrors `delete(int, int)`.
    pub fn delete(user_id: i32, group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM groups_edit_sessions WHERE user_id = {user_id} AND group_id = {group_id}"
        ));
    }

    /// Mirrors `deleteGroupWidgets(int)`.
    pub fn delete_group_widgets(group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_stickers WHERE group_id = {group_id} AND user_id = 0"
        ));
    }

    /// Mirrors `pickupUserWidgets(int)`.
    pub fn pickup_user_widgets(group_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_stickers SET group_id = 0 WHERE group_id = {group_id} AND user_id <> 0"
        ));
    }
}
