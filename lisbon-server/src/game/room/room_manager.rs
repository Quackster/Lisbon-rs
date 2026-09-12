//! Mirrors `net.h4bbo.lisbon.game.room.RoomManager`.
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use parking_lot::Mutex;

use crate::dao::mysql::badge_dao::BadgeDao;
use crate::dao::mysql::room_dao::RoomDao;
use crate::dao::mysql::room_favourites_dao::RoomFavouritesDao;
use crate::dao::mysql::room_vote_dao::RoomVoteDao;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;
use crate::game::room::room::Room;

pub struct RoomManager {
    room_map: Mutex<HashMap<i32, Arc<Mutex<Room>>>>,
    room_entry_badges: Mutex<HashMap<i32, Vec<String>>>,
}

impl RoomManager {
    /// Mirrors `PUBLIC_ROOM_OFFSET`.
    pub const PUBLIC_ROOM_OFFSET: i32 = 1000;

    /// Get the instance.
    pub fn get_instance() -> &'static RoomManager {
        static INSTANCE: OnceLock<RoomManager> = OnceLock::new();
        INSTANCE.get_or_init(|| RoomManager {
            room_map: Mutex::new(HashMap::new()),
            room_entry_badges: Mutex::new(BadgeDao::get_room_badges()),
        })
    }

    /// Mirrors `getRoomByModel(String)`.
    pub fn get_room_by_model(&self, model: &str) -> Option<Arc<Mutex<Room>>> {
        self.get_room_by_id(RoomDao::get_room_id_by_model(model))
    }

    /// Mirrors `getRoomById(int)`.
    pub fn get_room_by_id(&self, room_id: i32) -> Option<Arc<Mutex<Room>>> {
        if let Some(room) = self.room_map.lock().get(&room_id) {
            return Some(room.clone());
        }

        let Some(room) = RoomDao::get_room_by_id(room_id) else {
            return None;
        };

        let room = Arc::new(Mutex::new(room));
        self.room_map.lock().insert(room_id, room.clone());
        Some(room)
    }

    /// Mirrors `hasRoom(int)`.
    pub fn has_room(&self, room_id: i32) -> bool {
        self.room_map.lock().contains_key(&room_id)
    }

    /// Mirrors `removeRoom(int)`.
    pub fn remove_room(&self, room_id: i32) {
        self.room_map.lock().remove(&room_id);
    }

    /// Mirrors `addRoom(Room)`.
    pub fn add_room(&self, room: &Room) {
        let room_id = room.get_id();

        if self.room_map.lock().contains_key(&room_id) {
            return;
        }

        self.room_map
            .lock()
            .insert(room_id, Arc::new(Mutex::new(room.clone())));
    }

    /// Mirrors `getRoomEntryBadges()`.
    pub fn get_room_entry_badges(&self) -> HashMap<i32, Vec<String>> {
        self.room_entry_badges.lock().clone()
    }

    /// Mirrors `reloadBadges()`.
    pub fn reload_badges(&self) {
        *self.room_entry_badges.lock() = BadgeDao::get_room_badges();
    }

    /// Mirrors `giveBadges()`.
    pub fn give_badges(&self) {
        let entry_badges = self.room_entry_badges.lock();
        let room_map = self.room_map.lock();
        for (room_id, room_arc) in room_map.iter() {
            let Some(badges) = entry_badges.get(room_id) else {
                continue;
            };
            let room = room_arc.lock().clone();
            for player in room.get_entity_manager().get_players() {
                for badge in badges {
                    player.lock()
                        .get_badge_manager()
                        .try_add_badge(badge, None, 0);
                }
            }
        }
    }

    /// Mirrors `sortRooms(List<Room>)` (descending by current visitors).
    pub fn sort_rooms(&self, room_list: &mut Vec<Room>) {
        room_list.sort_by(|a, b| {
            b.get_data()
                .get_visitors_now()
                .cmp(&a.get_data().get_visitors_now())
        });
    }

    /// Mirrors `replaceQueryRooms(List<Room>)`.
    pub fn replace_query_rooms(&self, query_rooms: Vec<Room>) -> Vec<Room> {
        let mut room_list = Vec::new();

        for room in query_rooms {
            if self.room_map.lock().contains_key(&room.get_id()) {
                if let Some(cached) = self.get_room_by_id(room.get_data().get_id()) {
                    room_list.push(cached.lock().clone());
                }
            } else {
                room_list.push(room);
            }
        }

        room_list
    }

    /// Mirrors `getFavouriteRooms(int)`.
    pub fn get_favourite_rooms(&self, user_id: i32) -> Vec<Room> {
        let mut room_ids = RoomFavouritesDao::get_favourite_rooms(user_id);
        room_ids.reverse(); // To most recent favourite added at the top

        let mut rooms = Vec::new();

        for room_id in room_ids {
            if let Some(room) = self.get_room_by_id(room_id) {
                rooms.push(room.lock().clone());
            }
        }

        rooms
    }

    /// Mirrors `ratingSantiyCheck(List<Room>)`.
    pub fn rating_santiy_check(&self, room_list: &[Room]) {
        for room in room_list {
            if room.is_public_room() {
                continue;
            }

            if room.get_data().get_visitors_now() > 0 {
                continue;
            }

            if !(room.get_data().get_rating() > 0) {
                return;
            }

            RoomVoteDao::remove_expired_votes(room.get_id());
            let mut new_rating: i32 = RoomVoteDao::get_ratings(room.get_id())
                .values()
                .sum();

            if new_rating < 0 {
                new_rating = 0;
            }

            if new_rating != room.get_data().get_rating() {
                RoomDao::save_rating(room.get_id(), new_rating);
            }
        }
    }

    /// Mirrors `getRooms()`.
    pub fn get_rooms(&self) -> Vec<Arc<Mutex<Room>>> {
        self.room_map.lock().values().cloned().collect()
    }

    /// Mirrors `getChildRooms(Room)`.
    pub fn get_child_rooms(&self, room: &Room) -> Vec<Arc<Mutex<Room>>> {
        let mut room_list: Vec<Arc<Mutex<Room>>> = Vec::new();

        if room.is_public_room() {
            self.get_sub_rooms(room.get_id(), &mut room_list);

            for child in room_list.clone() {
                self.get_sub_rooms(child.lock().get_id(), &mut room_list);
            }
        }

        room_list
    }

    /// Mirrors `getSubRooms(int, List<Room>)` (the Java `null` entry for a
    /// missing room is skipped).
    fn get_sub_rooms(&self, id: i32, room_list: &mut Vec<Arc<Mutex<Room>>>) {
        for walkway in WalkwaysManager::get_instance().get_walkways() {
            if walkway.get_room_target_id() == id && walkway.get_room_id() != id {
                let already_present = room_list.iter().any(|r| {
                    let room = r.lock();
                    room.get_id() == walkway.get_room_id()
                        || room.get_id() == walkway.get_room_target_id()
                });

                if !already_present {
                    if let Some(room) = self.get_room_by_id(walkway.get_room_id()) {
                        room_list.push(room);
                    }
                }
            }
        }
    }
}
