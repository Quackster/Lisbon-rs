//! Mirrors `org.alexdev.http.dao.RegisterDao`.

use lisbon_server::dao::storage::Storage;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct RegisterDao;

impl RegisterDao {
    /// Mirrors `newUser(String, String, String, String, String, String)`.
    pub fn new_user(username: &str, password: &str, figure: &str, gender: &str, email: &str, birthday: &str) -> i32 {
        Storage::get_storage()
            .execute_insert(&format!(
                "INSERT INTO users (username, password, figure, sex, pool_figure, sso_ticket, email, birthday) VALUES ('{u}', '{p}', '{f}', '{g}', '', '', '{e}', '{b}')",
                u = escape(username),
                p = escape(password),
                f = escape(figure),
                g = escape(gender),
                e = escape(email),
                b = escape(birthday)
            ))
            .map(|id| id as i32)
            .unwrap_or(-1)
    }
}
