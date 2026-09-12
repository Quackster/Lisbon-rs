//! Mirrors `net.h4bbo.lisbon.game.events.Event`.

use crate::game::events::events_manager::EventsManager;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room_data::RoomData;
use crate::game::room::room_manager::RoomManager;
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Event {
    room_id: i32,
    user_id: i32,
    category_id: i32,
    name: String,
    description: String,
    expire: i64,
}

impl Event {
    /// Mirrors the 6-arg `Event(int, int, int, String, String, long)` constructor
    /// (Java stores `started` in `expire`).
    pub fn new(
        room_id: i32,
        user_id: i32,
        category_id: i32,
        name: String,
        description: String,
        expire: i64,
    ) -> Self {
        Self {
            room_id,
            user_id,
            category_id,
            name,
            description,
            expire,
        }
    }

    /// Mirrors `isExpired()`.
    pub fn is_expired(&self) -> bool {
        DateUtil::get_current_time_seconds() as i64 > self.expire
    }

    /// Mirrors `getRoomId()`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `getPlayersInEvent()`.
    pub fn get_players_in_event(&self) -> i32 {
        match RoomManager::get_instance().get_room_by_id(self.room_id) {
            Some(room) => room.lock().get_entity_manager().get_players().len() as i32,
            None => 0,
        }
    }

    /// Mirrors `getRoomData()`.
    pub fn get_room_data(&self) -> Option<RoomData> {
        RoomManager::get_instance()
            .get_room_by_id(self.room_id)
            .map(|room| room.lock().get_data().clone())
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getUserInfo()`.
    pub fn get_user_info(&self) -> Option<PlayerDetails> {
        PlayerManager::get_instance().get_player_data_by_id(self.user_id)
    }

    /// Mirrors `getCategoryId()`.
    pub fn get_category_id(&self) -> i32 {
        self.category_id
    }

    /// Mirrors `setCategoryId(int)`.
    pub fn set_category_id(&mut self, category_id: i32) {
        self.category_id = category_id;
    }

    /// Mirrors `getEventHoster()`.
    pub fn get_event_hoster(&self) -> Option<PlayerDetails> {
        PlayerManager::get_instance().get_player_data_by_id(self.user_id)
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `setName(String)`.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Mirrors `getDescription()`.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Mirrors `setDescription(String)`.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Mirrors `getExpireTime()`.
    pub fn get_expire_time(&self) -> i64 {
        self.expire
    }

    /// Mirrors `getStartedDate()`.
    pub fn get_started_date(&self) -> Option<String> {
        DateUtil::get_date_as_string(self.expire - EventsManager::get_event_lifetime())
    }
}
