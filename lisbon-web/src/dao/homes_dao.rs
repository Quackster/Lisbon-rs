//! Mirrors `org.alexdev.http.dao.HomesDao`.

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::homes::home::Home;

pub struct HomesDao;

impl HomesDao {
    /// Mirrors `create(int)`.
    pub fn create(user_id: i32) {
        Storage::get_storage()
            .execute(&format!("INSERT INTO homes_details (user_id) VALUES ({user_id})"));
    }

    /// Mirrors `getHome(int)`.
    pub fn get_home(user_id: i32) -> Option<Home> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM homes_details WHERE user_id = {user_id}"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `saveBackground(int, String)`.
    pub fn save_background(user_id: i32, background: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE homes_details SET background = '{b}' WHERE user_id = {user_id}",
            b = escape(background)
        ));
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> Home {
        let user_id = row.i32("user_id").unwrap_or(0);
        let background = row.str("background").unwrap_or_default();

        Home::new(user_id, &background)
    }
}
