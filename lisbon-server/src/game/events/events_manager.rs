//! Mirrors `net.h4bbo.lisbon.game.events.EventsManager`.

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use parking_lot::RwLock;

use crate::dao::mysql::events_dao::EventsDao;
use crate::game::entity::entity::Entity;
use crate::game::events::event::Event;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<EventsManager>>> = RwLock::new(None);
}

pub struct EventsManager {
    event_list: Mutex<Vec<Event>>,
}

impl EventsManager {
    fn new() -> Self {
        EventsDao::remove_expired_events();

        let manager = Self {
            event_list: Mutex::new(EventsDao::get_events()),
        };
        manager.remove_expired_events();
        manager
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }

    /// Mirrors `createEvent(Player, int, String, String)`.
    pub fn create_event(
        &self,
        player: &Player,
        category: i32,
        name: &str,
        description: &str,
    ) -> Event {
        let expire_time =
            DateUtil::get_current_time_seconds() as i64 + Self::get_event_lifetime();

        let room_id = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
            .map(|room| room.get_id())
            .unwrap_or(0);

        let event = Event::new(
            room_id,
            player.get_details().get_id(),
            category,
            name.to_string(),
            description.to_string(),
            expire_time,
        );

        EventsDao::add_event(
            event.get_room_id(),
            event.get_user_id(),
            event.get_category_id(),
            event.get_name(),
            event.get_description(),
            event.get_expire_time(),
        );

        self.event_list.lock().push(event.clone());
        event
    }

    /// Mirrors `canCreateEvent(Player)`.
    pub fn can_create_event(&self, player: &Player) -> bool {
        let room: Room = match player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        {
            Some(room) => room,
            None => return false,
        };

        if room.is_public_room() {
            return false;
        }

        if !room.is_owner(player.get_details().get_id()) {
            return false;
        }

        if Self::get_instance().is_hosting_event(player.get_details().get_id()) {
            return false;
        }

        if Self::get_instance().has_event(room.get_id()) {
            return false;
        }

        true
    }

    /// Mirrors `getEvents(int)`.
    pub fn get_events(&self, category_id: i32) -> Vec<Event> {
        let events = self.event_list.lock();

        if category_id == 0 {
            // 0 is the hottest events.
            let mut list: Vec<Event> = events
                .iter()
                .filter(|event| !event.is_expired())
                .cloned()
                .collect();
            list.sort_by(|a, b| b.get_players_in_event().cmp(&a.get_players_in_event()));
            return list;
        }

        events
            .iter()
            .filter(|event| event.get_category_id() == category_id && !event.is_expired())
            .cloned()
            .collect()
    }

    /// Mirrors `hasEvent(int)`.
    pub fn has_event(&self, room_id: i32) -> bool {
        self.get_event_by_room_id(room_id).is_some()
    }

    /// Mirrors `getEventByRoomId(int)`.
    pub fn get_event_by_room_id(&self, room_id: i32) -> Option<Event> {
        self.event_list
            .lock()
            .iter()
            .find(|event| event.get_room_id() == room_id && !event.is_expired())
            .cloned()
    }

    /// Mirrors `isHostingEvent(int)`.
    pub fn is_hosting_event(&self, user_id: i32) -> bool {
        self.event_list
            .lock()
            .iter()
            .any(|event| event.get_user_id() == user_id)
    }

    /// Mirrors `removeExpiredEvents()`.
    pub fn remove_expired_events(&self) {
        let expired_events: Vec<Event> = self
            .event_list
            .lock()
            .iter()
            .filter(|event| event.is_expired())
            .cloned()
            .collect();
        EventsDao::remove_events(&expired_events);
        self.event_list.lock().retain(|event| !event.is_expired());
    }

    /// Mirrors `getEventLifetime()`.
    pub fn get_event_lifetime() -> i64 {
        GameConfiguration::get_instance()
            .get_integer("events.expiry.minutes") as i64
            * 60
    }

    /// Mirrors `removeEvent(Event)`.
    pub fn remove_event(&self, event: &Event) {
        self.event_list
            .lock()
            .retain(|e| e.get_room_id() != event.get_room_id());
        EventsDao::remove_event(event);
    }

    /// Mirrors `getEventList()`.
    pub fn get_event_list(&self) -> Vec<Event> {
        self.event_list
            .lock()
            .iter()
            .filter(|event| !event.is_expired())
            .cloned()
            .collect()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<EventsManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }
}
