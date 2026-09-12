//! Mirrors `org.alexdev.http.dao.EmailDao`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::game::player::player_details::PlayerDetails;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct EmailDao;

impl EmailDao {
    /// Mirrors `exists(int, String)`.
    pub fn exists(id: i32, activation_code: &str) -> bool {
        for _row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_statistics WHERE user_id = {id} AND activation_code = '{}' LIMIT 1",
                escape(activation_code)
            ),
        ) {
            return true;
        }

        false
    }

    /// Mirrors `recoveryExists(int, String)`.
    pub fn recovery_exists(id: i32, recovery_code: &str) -> bool {
        for _row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_statistics WHERE user_id = {id} AND forgot_password_code = '{}' LIMIT 1",
                escape(recovery_code)
            ),
        ) {
            return true;
        }

        false
    }

    /// Mirrors `activate(int, String)`.
    pub fn activate(user_id: i32, activation_code: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET activation_code = NULL WHERE user_id = {user_id} AND activation_code = '{}' LIMIT 1",
            escape(activation_code)
        ));
    }

    /// Mirrors `removeRecoveryCode(int)`.
    pub fn remove_recovery_code(user_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET forgot_password_code = NULL, forgot_recovery_requested_time = NULL WHERE user_id = {user_id} LIMIT 1"
        ));
    }

    /// Mirrors `removeRecoveryCodeBatch()`.
    pub fn remove_recovery_code_batch() {
        Storage::get_storage().execute(
            "UPDATE users_statistics SET forgot_password_code = NULL, forgot_recovery_requested_time = NULL WHERE forgot_recovery_requested_time < UNIX_TIMESTAMP(DATE_SUB(NOW(), INTERVAL 1 DAY))",
        );
    }

    /// Mirrors `hasUserTradePass(int, String)`.
    pub fn has_user_trade_pass(user_id: i32, email: &str) -> bool {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users INNER JOIN users_statistics ON users.id = users_statistics.user_id WHERE email = '{}' AND id <> {user_id}",
                escape(email)
            ),
        ) {
            return row.str("activation_code").is_none() && row.bool("trade_enabled").unwrap_or(false);
        }

        false
    }

    /// Mirrors `getDetails(String, String)`.
    pub fn get_details(username: &str, email: &str) -> Option<PlayerDetails> {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users WHERE username = '{}' AND email = '{}' LIMIT 1",
                escape(username),
                escape(email)
            ),
        ) {
            let mut details = PlayerDetails::new();
            PlayerDao::fill(&mut details, &row);
            return Some(details);
        }

        None
    }

    /// Mirrors `getDetailsByEmail(String)`.
    pub fn get_details_by_email(email: &str) -> bool {
        for _row in Storage::get_storage()
            .query_all(&format!("SELECT id FROM users WHERE email = '{}' LIMIT 1", escape(email)))
        {
            return true;
        }

        false
    }
}
