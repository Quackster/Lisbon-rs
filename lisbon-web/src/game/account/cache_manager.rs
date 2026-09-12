//! Mirrors `org.alexdev.http.game.account.CacheManager`.

use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::web_connection::{WebConnection, SessionValue};

pub struct CacheManager;

impl CacheManager {
    /// Mirrors `savePage(WebConnection, String, String, int)`.
    pub fn save_page(connection: &WebConnection, page_name: &str, page: &str, max_lifetime_seconds: i32) {
        connection.session().set(
            &format!("savedCache{page_name}Time"),
            SessionValue::Str(
                (DateUtil::get_current_time_seconds() + max_lifetime_seconds).to_string(),
            ),
        );
        connection
            .session()
            .set(&format!("savedCache{page_name}Source"), SessionValue::Str(page.to_string()));
    }

    /// Mirrors `deletePage(WebConnection, String)`.
    pub fn delete_page(connection: &WebConnection, page_name: &str) {
        connection.session().delete(&format!("savedCache{page_name}Time"));
        connection.session().delete(&format!("savedCache{page_name}Source"));
    }

    /// Mirrors `getPage(WebConnection, String)`.
    pub fn get_page(connection: &WebConnection, page_name: &str) -> Option<String> {
        connection.session().get_string(&format!("savedCache{page_name}Source"))
    }

    /// Mirrors `useCachePage(WebConnection, String)`.
    /// Port note: the Java `getLong` try/catch degrades to `get_int` (an
    // unparseable value reads as `0`, never expiring the cache).
    pub fn use_cache_page(connection: &WebConnection, page_name: &str) -> bool {
        if connection.session().contains(&format!("savedCache{page_name}Time")) {
            let expire = connection.session().get_int(&format!("savedCache{page_name}Time")) as i64;

            if (DateUtil::get_current_time_seconds() as i64) < expire {
                return true;
            }
        }

        false
    }
}
