//! Mirrors `org.alexdev.http.dao.GuestbookDao`.

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::homes::guestbook_entry::GuestbookEntry;

pub struct GuestbookDao;

impl GuestbookDao {
    /// Mirrors `create(int, int, int, String)`.
    pub fn create(user_id: i32, home_id: i32, group_id: i32, message: &str) -> GuestbookEntry {
        let guestbook_id = Storage::get_storage()
            .execute_insert(&format!(
                "INSERT INTO cms_guestbook_entries (user_id, home_id, group_id, message) VALUES ({user_id}, {home_id}, {group_id}, '{m}')",
                m = escape(message)
            ))
            .map(|id| id as i32)
            .unwrap_or(-1);
        let guestbook_creation = DateUtil::get_current_time_seconds() as i64;

        GuestbookEntry::new(guestbook_id, user_id, home_id, group_id, message, guestbook_creation)
    }

    /// Mirrors `remove(int, int, int)`.
    pub fn remove(id: i32, home_id: i32, group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_guestbook_entries WHERE id = {id} AND home_id = {home_id} AND group_id = {group_id}"
        ));
    }

    /// Mirrors `getEntry(int)`.
    pub fn get_entry(id: i32) -> Option<GuestbookEntry> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_guestbook_entries WHERE id = {id}"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getEntriesByHome(int)`.
    pub fn get_entries_by_home(home_id: i32) -> Vec<GuestbookEntry> {
        let mut entries = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_guestbook_entries WHERE home_id = {home_id} ORDER BY created_at DESC LIMIT 500"))
        {
            entries.push(Self::fill(&row));
        }

        entries
    }

    /// Mirrors `getEntriesByGroup(int)`.
    pub fn get_entries_by_group(group_id: i32) -> Vec<GuestbookEntry> {
        let mut entries = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_guestbook_entries WHERE group_id = {group_id} ORDER BY created_at DESC LIMIT 500"))
        {
            entries.push(Self::fill(&row));
        }

        entries
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> GuestbookEntry {
        GuestbookEntry::new(
            row.i32("id").unwrap_or(0),
            row.i32("user_id").unwrap_or(0),
            row.i32("home_id").unwrap_or(0),
            row.i32("group_id").unwrap_or(0),
            &row.str("message").unwrap_or_default(),
            row.i64("created_at").unwrap_or(0),
        )
    }
}
