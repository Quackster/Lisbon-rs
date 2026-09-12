//! Mirrors `net.h4bbo.lisbon.game.room.managers.RoomTimerManager`.
use crate::game::entity::entity::Entity;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

pub struct RoomTimerManager {
    // Port note: `RoomEntity.getEntity()` is not ported (the Rust
    // `RoomEntityState` guard cannot hand out a borrowed entity from a
    // temporary `MutexGuard`); the field is always `None`.
    entity: Option<Box<dyn Entity + Send>>,
    room_entity: RoomEntity,
    look_timer: i32,
    afk_timer: i64,
    sleep_timer: i64,
    chat_bubble_timer: i64,
}

impl RoomTimerManager {
    /// Mirrors the `RoomTimerManager(RoomEntity)` constructor.
    pub fn new(room_entity: &RoomEntity) -> Self {
        Self {
            entity: None,
            room_entity: room_entity.clone(),
            look_timer: 0,
            afk_timer: 0,
            sleep_timer: 0,
            chat_bubble_timer: 0,
        }
    }

    /// Mirrors `resetTimers()`.
    pub fn reset_timers(&mut self) {
        self.reset_room_timer();
        self.stop_chat_bubble_timer();
        self.stop_look_timer();
    }

    /// Mirrors `resetRoomTimer()` (the no-arg Java overload; the 10 minute
    // default comes from `afk.timer.seconds`).
    pub fn reset_room_timer(&mut self) {
        self.reset_room_timer_secs(
            GameConfiguration::get_instance().get_integer("afk.timer.seconds") as i64,
        );
    }

    /// Mirrors `resetRoomTimer(long)` (the Java overload).
    pub fn reset_room_timer_secs(&mut self, afk_timer: i64) {
        let now = DateUtil::get_current_time_seconds() as i64;
        self.afk_timer = now + afk_timer;
        self.sleep_timer =
            now + GameConfiguration::get_instance().get_integer("sleep.timer.seconds") as i64;

        if self.room_entity.contains_status(StatusType::AvatarSleep) {
            self.room_entity.remove_status(StatusType::AvatarSleep);
            self.room_entity.set_needs_update(true);
        }
    }

    /// Mirrors `beginLookTimer()`.
    pub fn begin_look_timer(&mut self) {
        self.look_timer = DateUtil::get_current_time_seconds() + 6;
    }

    /// Mirrors `stopLookTimer()`.
    pub fn stop_look_timer(&mut self) {
        self.look_timer = -1;
    }

    /// Mirrors `beginChatBubbleTimer()`.
    pub fn begin_chat_bubble_timer(&mut self) {
        let timeout = GameConfiguration::get_instance().get_integer("chat.bubble.timeout.seconds");

        if timeout > 0 {
            self.chat_bubble_timer = DateUtil::get_current_time_seconds() as i64 + timeout as i64;
        }
    }

    /// Mirrors `stopChatBubbleTimer()`.
    pub fn stop_chat_bubble_timer(&mut self) {
        self.chat_bubble_timer = -1;
    }

    /// Mirrors `getEntity()`.
    pub fn get_entity(&self) -> Option<&(dyn Entity + Send)> {
        self.entity.as_deref()
    }

    /// Mirrors `getChatBubbleTimer()`.
    pub fn get_chat_bubble_timer(&self) -> i64 {
        self.chat_bubble_timer
    }

    /// Mirrors `getLookTimer()`.
    pub fn get_look_timer(&self) -> i32 {
        self.look_timer
    }

    /// Mirrors `getAfkTimer()`.
    pub fn get_afk_timer(&self) -> i64 {
        self.afk_timer
    }

    /// Mirrors `getSleepTimer()`.
    pub fn get_sleep_timer(&self) -> i64 {
        self.sleep_timer
    }
}
