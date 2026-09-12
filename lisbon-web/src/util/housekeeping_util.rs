//! Mirrors `org.alexdev.http.util.HousekeepingUtil`.

use lisbon_server::dao::mysql::room_dao::RoomDao;

use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

/// Mirrors `org.alexdev.http.util.HousekeepingUtil`.
#[derive(serde::Serialize)]
pub struct HousekeepingUtil;

impl HousekeepingUtil {
    /// Mirrors `getRoomName(int)`.
    pub fn get_room_name(&self, room_id: i32) -> String {
        match RoomDao::get_room_by_id(room_id) {
            None => "ERROR".to_string(),
            Some(room) => room.get_data().get_name().to_string(),
        }
    }

    /// Mirrors `formatNewsStory(String)`.
    pub fn format_news_story(&self, fullstory: &str) -> String {
        BBCode::format(&HtmlUtil::escape(&BBCode::normalise(fullstory)), true)
    }
}
