//! Mirrors `org.alexdev.http.dao.MinimailDao`.

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::minimail::minimail_message::MinimailMessage;

pub struct MinimailDao;

impl MinimailDao {
    /// Mirrors `getMessage(int, int)`.
    pub fn get_message(message_id: i32, target_id: i32) -> Option<MinimailMessage> {
        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_minimail WHERE id = {message_id} AND (target_id = {target_id} OR sender_id = {target_id})"),
        ) {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getMessages(int)`.
    pub fn get_messages(user_id: i32) -> Vec<MinimailMessage> {
        let mut messages = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_minimail WHERE to_id = {user_id} AND is_trash = 0 AND is_deleted = 0 AND target_id = {user_id}"),
        ) {
            messages.push(Self::fill(&row));
        }

        messages
    }

    /// Mirrors `getMessagesSent(int)`.
    pub fn get_messages_sent(user_id: i32) -> Vec<MinimailMessage> {
        let mut messages = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_minimail WHERE sender_id = {user_id} AND is_trash = 0 AND is_deleted = 0 AND target_id = {user_id}"),
        ) {
            messages.push(Self::fill(&row));
        }

        messages
    }

    /// Mirrors `createMessages(List<MinimailMessage>)`.
    // Port note: the JDBC batch (`addBatch`/`executeBatch`) is executed as
    // one insert per message; the `setAutoCommit(false)` transaction wrapper
    // has no equivalent in the synchronous `Storage` API.
    pub fn create_messages(messages: &[MinimailMessage]) {
        for message in messages {
            Storage::get_storage().execute(&format!(
                "INSERT INTO cms_minimail (target_id, sender_id, to_id, subject, message, conversation_id) VALUES ({t}, {s}, {to}, '{sub}', '{m}', {c})",
                t = message.target_id,
                s = message.sender_id,
                to = message.to_id,
                sub = escape(&message.subject),
                m = escape(&message.message),
                c = message.conversation_id
            ));
        }
    }

    /// Mirrors `updateMessage(MinimailMessage)`.
    pub fn update_message(message: &MinimailMessage) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_minimail SET is_trash = {trash}, conversation_id = {c}, is_read = {read} WHERE id = {id}",
            trash = message.is_trash as i32,
            c = message.conversation_id,
            read = message.is_read as i32,
            id = message.id
        ));
    }

    /// Mirrors `deleteMessage(MinimailMessage)`.
    pub fn delete_message(message: &MinimailMessage) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_minimail SET is_deleted = 1 WHERE id = {} AND target_id = {}",
            message.id,
            message.target_id
        ));
    }

    /// Mirrors `emptyTrash(int)`.
    pub fn empty_trash(user_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_minimail SET is_deleted = 1 WHERE is_trash = 1 AND target_id = {user_id}"
        ));
    }

    /// Mirrors `getMessagesConversation(int, int)`.
    pub fn get_messages_conversation(user_id: i32, conversation_id: i32) -> Vec<MinimailMessage> {
        let mut messages = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM cms_minimail WHERE conversation_id = {conversation_id} AND target_id = {user_id} OR (sender_id = {user_id} AND id = {conversation_id})"),
        ) {
            messages.push(Self::fill(&row));
        }

        messages
    }

    /// Mirrors `getMessageTrash(int)`.
    pub fn get_message_trash(user_id: i32) -> Vec<MinimailMessage> {
        let mut messages = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM cms_minimail WHERE is_trash = 1 AND is_deleted = 0 AND target_id = {user_id}"))
        {
            messages.push(Self::fill(&row));
        }

        messages
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> MinimailMessage {
        MinimailMessage::new(
            row.i32("id").unwrap_or(0),
            row.i32("target_id").unwrap_or(0),
            row.i32("to_id").unwrap_or(0),
            row.i32("sender_id").unwrap_or(0),
            row.bool("is_read").unwrap_or(false),
            &row.str("subject").unwrap_or_default(),
            &row.str("message").unwrap_or_default(),
            row.i64("date_sent").unwrap_or(0),
            row.i32("conversation_id").unwrap_or(0),
            row.bool("is_trash").unwrap_or(false),
        )
    }
}
