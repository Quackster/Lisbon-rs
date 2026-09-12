//! Mirrors `org.alexdev.http.dao.VerifyDao`.

use lisbon_server::dao::storage::Storage;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct VerifyDao;

impl VerifyDao {
    /// Mirrors `getName(String)`.
    pub fn get_name(verify_code: &str) -> Option<String> {
        Storage::get_storage().get_string(
            &format!(
                "SELECT username FROM users_statistics INNER JOIN users ON users_statistics.user_id = users.id WHERE verify_code = '{}' LIMIT 1",
                escape(verify_code)
            ),
            "username",
        )
    }

    /// Mirrors `clearName(String)`.
    // Port note: the Java `throws SQLException` is dropped (errors are
    // logged by `Storage`).
    pub fn clear_name(verify_code: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET verify_code = NULL WHERE verify_code = '{}' LIMIT 1",
            escape(verify_code)
        ));
    }
}
