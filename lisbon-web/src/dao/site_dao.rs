//! Mirrors `org.alexdev.http.dao.SiteDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::util::date_util::DateUtil;

pub struct SiteDao;

impl SiteDao {
    /// Mirrors `getLastVisits()`.
    // Port note: the Java `throws SQLException` is dropped (errors are
    // logged by `Storage`).
    pub fn get_last_visits() -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) as count FROM users WHERE UNIX_TIMESTAMP(last_online) > {}",
                DateUtil::get_current_time_seconds() as i64 - 2592000
            ),
        ) {
            if let Some(value) = row.i32("count") {
                count = value;
            }
        }

        count
    }
}
