//! Mirrors `org.alexdev.http.dao.SessionDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct SessionDao;

impl SessionDao {
    /// Mirrors `getRememberToken(String)`.
    pub fn get_remember_token(token: &str) -> i32 {
        let mut user_id = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT id FROM users WHERE remember_token = '{}'", escape(token)))
        {
            if let Some(value) = row.i32("id") {
                user_id = value;
            }
        }

        user_id
    }

    /// Mirrors `setRememberToken(int, String)`.
    pub fn set_remember_token(user_id: i32, token: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET remember_token = '{t}' WHERE id = {user_id}",
            t = escape(token)
        ));
    }

    /// Mirrors `clearRememberToken(int)`.
    pub fn clear_remember_token(user_id: i32) {
        Storage::get_storage()
            .execute(&format!("UPDATE users SET remember_token = NULL WHERE id = {user_id}"));
    }

    /// Mirrors `savePreferences(String, boolean, boolean, boolean, boolean, boolean, int)`.
    pub fn save_preferences(
        motto: &str,
        profile_visibility: bool,
        show_online_status: bool,
        word_filter_enabled: bool,
        allow_friend_requests: bool,
        allow_stalking: bool,
        user_id: i32,
    ) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET motto = '{m}', profile_visible = {pv}, online_status_visible = {osv}, wordfilter_enabled = {wfe}, allow_friend_requests = {afr}, allow_stalking = {asv} WHERE id = {user_id}",
            m = escape(motto),
            pv = profile_visibility as i32,
            osv = show_online_status as i32,
            wfe = word_filter_enabled as i32,
            afr = allow_friend_requests as i32,
            asv = allow_stalking as i32
        ));
    }

    /// Mirrors `saveTrade(int, boolean)`.
    pub fn save_trade(user_id: i32, trade_setting: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET trade_enabled = {ts} WHERE id = {user_id}",
            ts = trade_setting as i32
        ));
    }
}
