//! Mirrors `net.h4bbo.lisbon.game.moderation.ChatMessage`.

use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    player_id: i32,
    message: String,
    chat_message_type: ChatMessageType,
    room_id: i32,
    sent_time: i64,
}

impl ChatMessage {
    /// Mirrors the `ChatMessage(int, String, ChatMessageType, int, long)`
    /// constructor.
    pub fn new(
        player_id: i32,
        message: &str,
        chat_message_type: ChatMessageType,
        room_id: i32,
        sent_time: i64,
    ) -> Self {
        Self {
            player_id,
            message: message.to_string(),
            chat_message_type,
            room_id,
            sent_time,
        }
    }

    /// Mirrors `getPlayerId`.
    pub fn get_player_id(&self) -> i32 {
        self.player_id
    }

    /// Mirrors `getRoomId`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `getMessage`.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Mirrors `getChatMessageType`.
    pub fn get_chat_message_type(&self) -> ChatMessageType {
        self.chat_message_type
    }

    /// Mirrors `getSentTime`.
    pub fn get_sent_time(&self) -> i64 {
        self.sent_time
    }
}
