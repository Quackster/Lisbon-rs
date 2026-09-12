//! Mirrors `net.h4bbo.lisbon.dao.mysql.EventsDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::events::event::Event;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct EventsDao;

impl EventsDao {
    /// Mirrors `addEvent(int, int, int, String, String, long)`.
    pub fn add_event(
        room_id: i32,
        user_id: i32,
        category_id: i32,
        name: &str,
        description: &str,
        expire_time: i64,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO rooms_events (room_id, user_id, category_id, name, description, expire_time) VALUES ({room_id}, {user_id}, {category_id}, '{}', '{}', {expire_time})",
            escape(name),
            escape(description)
        ));
    }

    /// Mirrors `removeEvent(Event)`.
    pub fn remove_event(event: &Event) {
        Self::remove_events(&[event.clone()]);
    }

    /// Mirrors `removeEvents(List<Event>)`.
    pub fn remove_events(event_list: &[Event]) {
        for event in event_list {
            Storage::get_storage().execute(&format!(
                "DELETE FROM rooms_events WHERE room_id = {}",
                event.get_room_id()
            ));
        }
    }

    /// Mirrors `removeExpiredEvents()`.
    pub fn remove_expired_events() {
        let now = DateUtil::get_current_time_seconds() as i64;

        Storage::get_storage().execute(&format!(
            "DELETE FROM rooms_events WHERE expire_time < {now}"
        ));
    }

    /// Mirrors `getEvents()`.
    pub fn get_events() -> Vec<Event> {
        let mut event_map = Vec::new();
        let now = DateUtil::get_current_time_seconds() as i64;

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM rooms_events WHERE expire_time > {now}"),
        ) {
            if let (Some(room_id), Some(user_id), Some(category_id), Some(name), Some(description), Some(expire_time)) = (
                row.i32("room_id"),
                row.i32("user_id"),
                row.i32("category_id"),
                row.str("name"),
                row.str("description"),
                row.i64("expire_time"),
            ) {
                event_map.push(Event::new(
                    room_id,
                    user_id,
                    category_id,
                    name,
                    description,
                    expire_time,
                ));
            }
        }

        event_map
    }

    /// Mirrors `save(Event)`.
    pub fn save(event: &Event) {
        Storage::get_storage().execute(&format!(
            "UPDATE rooms_events SET category_id = {}, name = '{}', description = '{}' WHERE room_id = {}",
            event.get_category_id(),
            escape(event.get_name()),
            escape(event.get_description()),
            event.get_room_id()
        ));
    }
}
