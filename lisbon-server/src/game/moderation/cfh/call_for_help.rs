//! Mirrors `net.h4bbo.lisbon.game.moderation.cfh.CallForHelp`.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug)]
pub struct CallForHelp {
    cry_id: i32,
    caller_id: i32,
    message: String,
    picked_up_by: i32,
    room: Room,
    request_time: i64,
    category: i32,
    expire_time: i64,
    is_deleted: bool,
}

impl CallForHelp {
    /// Mirrors the package-private `CallForHelp(int, int, Room, String)`
    /// constructor (the 30-minute expiry is `TimeUnit.MINUTES.toSeconds(30)`).
    pub fn new(cry_id: i32, caller_id: i32, room: Room, message: &str) -> Self {
        let request_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);

        Self {
            cry_id,
            caller_id,
            message: message.to_string(),
            picked_up_by: 0,
            room,
            request_time,
            category: 2,
            expire_time: DateUtil::get_current_time_seconds() as i64 + 1800,
            is_deleted: false,
        }
    }

    /// Mirrors `getMessage`.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Mirrors `getPickedUpBy`.
    pub fn get_picked_up_by(&self) -> i32 {
        self.picked_up_by
    }

    /// Mirrors `getRoom`.
    pub fn get_room(&self) -> &Room {
        &self.room
    }

    /// Mirrors `getCategory`.
    pub fn get_category(&self) -> i32 {
        self.category
    }

    /// Mirrors `getCaller`.
    pub fn get_caller(&self) -> i32 {
        self.caller_id
    }

    /// Mirrors `getCryId`.
    pub fn get_cry_id(&self) -> i32 {
        self.cry_id
    }

    /// Mirrors `isOpen`.
    pub fn is_open(&self) -> bool {
        self.picked_up_by == 0 && !self.is_deleted
    }

    /// Mirrors `getFormattedRequestTime` (the Java timestamp is in
    /// milliseconds; `DateUtil.get_date` takes seconds).
    pub fn get_formatted_request_time(&self) -> String {
        DateUtil::get_date(self.request_time / 1000, "HH:mm d/MM/yyy")
    }

    /// Mirrors `updateCategory`.
    pub fn update_category(&mut self, new_category: i32) {
        self.category = new_category;
    }

    /// Mirrors `setPickedUpBy`.
    pub fn set_picked_up_by(&mut self, moderator: &Player) {
        self.picked_up_by = moderator.get_details().get_id();
    }

    /// Mirrors `getExpireTime`.
    pub fn get_expire_time(&self) -> i64 {
        self.expire_time
    }

    /// Mirrors `setExpireTime`.
    pub fn set_expire_time(&mut self, expire_time: i64) {
        self.expire_time = expire_time
    }

    /// Mirrors `isDeleted`.
    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    /// Mirrors `setDeleted`.
    pub fn set_deleted(&mut self, deleted: bool) {
        self.is_deleted = deleted
    }
}
