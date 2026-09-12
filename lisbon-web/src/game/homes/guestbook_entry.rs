//! Mirrors `org.alexdev.http.game.homes.GuestbookEntry`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::date_util::DateUtil;

use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct GuestbookEntry {
    pub id: i32,
    pub user_id: i32,
    pub home_id: i32,
    pub group_id: i32,
    pub creation_date: i64,
    pub message: String,
}

impl GuestbookEntry {
    /// Mirrors the `GuestbookEntry(int, int, int, int, String, long)` constructor.
    pub fn new(
        id: i32,
        user_id: i32,
        home_id: i32,
        group_id: i32,
        message: &str,
        creation_date: i64,
    ) -> Self {
        Self {
            id,
            user_id,
            home_id,
            group_id,
            creation_date,
            message: message.to_string(),
        }
    }

    /// Mirrors `getUser()`.
    pub fn get_user(&self) -> Option<PlayerDetails> {
        PlayerDao::get_details(self.user_id)
    }

    /// Mirrors `getCreationDate()`.
    pub fn get_creation_date(&self) -> String {
        DateUtil::get_friendly_date(self.creation_date)
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> String {
        BBCode::format(
            &HtmlUtil::escape(&BBCode::normalise(&WordfilterManager::filter_sentence(&self.message))),
            false,
        )
    }
}
