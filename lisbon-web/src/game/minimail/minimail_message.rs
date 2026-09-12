//! Mirrors `org.alexdev.http.game.minimail.MinimailMessage`.

use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::date_util::DateUtil;

use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct MinimailMessage {
    pub id: i32,
    pub is_read: bool,
    pub target_id: i32,
    pub to_id: i32,
    pub sender_id: i32,
    pub subject: String,
    pub message: String,
    pub date_sent: i64,
    pub conversation_id: i32,
    pub is_trash: bool,
    pub target: Option<PlayerDetails>,
    pub author: Option<PlayerDetails>,
}

impl MinimailMessage {
    /// Mirrors the `MinimailMessage(int, int, int, int, boolean, String, String, long, int, boolean)` constructor.
    pub fn new(
        id: i32,
        target_id: i32,
        to_id: i32,
        sender_id: i32,
        is_read: bool,
        subject: &str,
        message: &str,
        date_sent: i64,
        conversation_id: i32,
        is_trash: bool,
    ) -> Self {
        Self {
            id,
            target_id,
            to_id,
            is_read,
            sender_id,
            subject: subject.to_string(),
            message: message.to_string(),
            date_sent,
            conversation_id,
            is_trash,
            target: None,
            author: None,
        }
    }

    /// Mirrors `setTargetId(int)`.
    pub fn set_target_id(&mut self, target_id: i32) {
        self.target_id = target_id;
    }

    /// Mirrors `setToId(int)`.
    pub fn set_to_id(&mut self, to_id: i32) {
        self.to_id = to_id;
    }

    /// Mirrors `setRead(boolean)`.
    pub fn set_read(&mut self, read: bool) {
        self.is_read = read;
    }

    /// Mirrors `setMessage(String)`.
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// Mirrors `setSenderId(int)`.
    pub fn set_sender_id(&mut self, sender_id: i32) {
        self.sender_id = sender_id;
    }

    /// Mirrors `setSubject(String)`.
    pub fn set_subject(&mut self, subject: &str) {
        self.subject = subject.to_string();
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> String {
        WordfilterManager::filter_sentence(&self.message)
    }

    /// Mirrors `getFormattedSubject()`.
    pub fn get_formatted_subject(&self) -> String {
        BBCode::format(&HtmlUtil::escape(&self.subject), false)
    }

    /// Mirrors `getFormattedMessage()`.
    pub fn get_formatted_message(&self) -> String {
        BBCode::format(
            &HtmlUtil::escape(&WordfilterManager::filter_sentence(&self.message)),
            false,
        )
    }

    /// Mirrors `getDate()`.
    pub fn get_date(&self) -> String {
        DateUtil::get_friendly_date(self.date_sent)
    }

    /// Mirrors `getIsoDate()`.
    pub fn get_iso_date(&self) -> String {
        DateUtil::get_date(self.date_sent, "yyyy-MM-dd'T'HH:mm:ssZ")
    }

    /// Mirrors `setConversationId(int)`.
    pub fn set_conversation_id(&mut self, conversation_id: i32) {
        self.conversation_id = conversation_id;
    }

    /// Mirrors `setTrash(boolean)`.
    pub fn set_trash(&mut self, trash: bool) {
        self.is_trash = trash;
    }

    /// Mirrors `setTarget(PlayerDetails)`.
    pub fn set_target(&mut self, target: Option<PlayerDetails>) {
        self.target = target;
    }

    /// Mirrors `setAuthor(PlayerDetails)`.
    pub fn set_author(&mut self, author: Option<PlayerDetails>) {
        self.author = author;
    }
}
