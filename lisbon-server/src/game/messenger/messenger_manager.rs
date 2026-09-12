//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerManager`.

use std::sync::OnceLock;

use crate::game::messenger::messenger::Messenger;
use crate::game::player::player_manager::PlayerManager;

pub struct MessengerManager;

impl MessengerManager {
    /// Mirrors `getMessengerData(int)`.
    pub fn get_messenger_data_by_id(&self, user_id: i32) -> Option<Messenger> {
        if let Some(player) = PlayerManager::get_instance().get_player_by_id(user_id) {
            return player.lock().get_messenger().cloned();
        }

        PlayerManager::get_instance()
            .get_player_data_by_id(user_id)
            .map(|details| Messenger::from_details(&details))
    }

    /// Mirrors `getMessengerData(String)`.
    pub fn get_messenger_data_by_name(&self, username: &str) -> Option<Messenger> {
        if let Some(player) = PlayerManager::get_instance().get_player_by_name(username) {
            return player.lock().get_messenger().cloned();
        }

        PlayerManager::get_instance()
            .get_player_data_by_name(username)
            .map(|details| Messenger::from_details(&details))
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static MessengerManager {
        static INSTANCE: OnceLock<MessengerManager> = OnceLock::new();
        INSTANCE.get_or_init(|| MessengerManager)
    }
}
