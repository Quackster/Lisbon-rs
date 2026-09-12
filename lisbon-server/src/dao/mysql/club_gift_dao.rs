//! Mirrors `net.h4bbo.lisbon.dao.mysql.ClubGiftDao`.

use crate::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct ClubGiftDao;

impl ClubGiftDao {
    /// Mirrors `getLastGift(int)` (the Java `Pair<Long, String>`; the date is
    /// returned as the epoch milliseconds from `getTime()`).
    pub fn get_last_gift(user_id: i32) -> Option<(i64, String)> {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_club_gifts WHERE user_id = {user_id} ORDER BY date_received DESC LIMIT 1"
            ),
        ) {
            if let (Some(date_received), Some(sprite)) = (
                row.i64("date_received"),
                row.str("sprite"),
            ) {
                return Some((date_received * 1000, sprite));
            }
        }

        None
    }

    /// Mirrors `incrementGiftData(long)`.
    pub fn increment_gift_data(next_gift_date: i64) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET gifts_due = gifts_due + 1, club_gift_due = FROM_UNIXTIME(UNIX_TIMESTAMP() + {next_gift_date}) WHERE CURRENT_TIMESTAMP() > club_gift_due"
        ));
    }

    /// Mirrors `addGift(int, String)`.
    pub fn add_gift(user_id: i32, sprite: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_club_gifts (user_id, sprite) VALUES ({user_id}, '{}')",
            escape(sprite)
        ));
    }

    /// Mirrors `clearGiftHistory(int)`.
    pub fn clear_gift_history(user_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_club_gifts WHERE user_id = {user_id}"
        ));
    }
}
