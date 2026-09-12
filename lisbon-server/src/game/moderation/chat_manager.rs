//! Mirrors `net.h4bbo.lisbon.game.moderation.ChatManager`.

use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::moderation::chat_message::ChatMessage;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::util::date_util::DateUtil;

/// Mirrors `ChatManager`.
///
/// The Java `BlockingQueue` (with `drainTo`) maps to a `parking_lot`
/// guarded `Vec` that is swapped out whole on save.
pub struct ChatManager {
    chat_message_queue: Mutex<Vec<ChatMessage>>,
}

impl ChatManager {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            chat_message_queue: Mutex::new(Vec::new()),
        }
    }

    /// Queue a message to be saved to the database.
    pub fn queue(
        &self,
        player: &Player,
        room: &Room,
        message: &str,
        chat_message_type: ChatMessageType,
    ) {
        self.chat_message_queue.lock().push(ChatMessage::new(
            player.get_details().get_id(),
            message,
            chat_message_type,
            room.get_id(),
            DateUtil::get_current_time_seconds() as i64,
        ));
    }

    /// Save all the queued chat messages.
    pub fn perform_chat_saving(&self) {
        let mut drained = Vec::new();
        let mut queue = self.chat_message_queue.lock();
        std::mem::swap(&mut *queue, &mut drained);
        RoomDao::save_chat_log(&drained);
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static ChatManager {
        static INSTANCE: OnceLock<ChatManager> = OnceLock::new();
        INSTANCE.get_or_init(ChatManager::new)
    }
}
