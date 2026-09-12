//! Mirrors `org.alexdev.http.game.groups.DiscussionReply`.

use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::date_util::DateUtil;

use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct DiscussionReply {
    pub id: i32,
    pub user_id: i32,
    pub is_new: bool,
    pub group_id: i32,
    pub message: String,
    pub figure: String,
    pub username: String,
    pub is_online: bool,
    pub equipped_badge: Option<String>,
    pub group_badge: Option<String>,
    pub forum_messages: i32,
    pub created_at: i64,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub modified_at: i64,
}

impl DiscussionReply {
    /// Mirrors the `DiscussionReply(int, int, String, String, String, boolean, String, int, String, int, boolean, boolean, Time, Time, boolean)` constructor
    /// (the `Time` arguments are passed as unix seconds).
    pub fn new(
        id: i32,
        user_id: i32,
        message: &str,
        figure: &str,
        username: &str,
        is_online: bool,
        equipped_badge: Option<String>,
        group_id: i32,
        group_badge: Option<String>,
        forum_messages: i32,
        is_edited: bool,
        is_deleted: bool,
        created_at: i64,
        modified_at: i64,
        has_read: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            is_new: !has_read,
            group_id,
            message: message.to_string(),
            figure: figure.to_string(),
            username: username.to_string(),
            is_online,
            equipped_badge,
            group_badge,
            forum_messages,
            created_at,
            is_edited,
            is_deleted,
            modified_at,
        }
    }

    /// Mirrors `getCreatedDate(String)`.
    pub fn get_created_date(&self, date_format: &str) -> String {
        DateUtil::get_date(self.created_at, date_format)
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getEditedDate(String)`.
    pub fn get_edited_date(&self, date_format: &str) -> String {
        DateUtil::get_date(self.modified_at, date_format)
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> String {
        WordfilterManager::filter_sentence(&self.message)
    }

    /// Mirrors `getFormattedMessage()`.
    pub fn get_formatted_message(&self) -> String {
        BBCode::format(
            &HtmlUtil::escape(&BBCode::normalise(&WordfilterManager::filter_sentence(&self.message))),
            false,
        )
    }

    /// Mirrors `hasBadge()`.
    pub fn has_badge(&self) -> bool {
        self.equipped_badge.is_some()
    }

    /// Mirrors `hasGroupBadge()`.
    pub fn has_group_badge(&self) -> bool {
        self.group_badge.is_some()
    }

    /// Mirrors `setMessage(String)`.
    pub fn set_message(&mut self, message: &str) {
        self.message = WordfilterManager::filter_sentence(message);
    }

    /// Mirrors `setEdited(boolean)`.
    pub fn set_edited(&mut self, edited: bool) {
        self.is_edited = edited;
    }

    /// Mirrors `setDeleted(boolean)`.
    pub fn set_deleted(&mut self, deleted: bool) {
        self.is_deleted = deleted;
    }
}
