//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomFavouritesDao`.

use crate::dao::storage::{RowGetters, Storage};

pub struct RoomFavouritesDao;

impl RoomFavouritesDao {
    /// Mirrors `getFavouriteRooms(int)`.
    pub fn get_favourite_rooms(user_id: i32) -> Vec<i32> {
        let mut room_ids = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT room_id FROM users_room_favourites WHERE user_id = {user_id}"))
        {
            if let Some(room_id) = row.i32("room_id") {
                room_ids.push(room_id);
            }
        }

        room_ids
    }

    /// Mirrors `addFavouriteRoom(int, int)`.
    pub fn add_favourite_room(user_id: i32, room_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_room_favourites (user_id, room_id) VALUES ({user_id}, {room_id})"
        ));
    }

    /// Mirrors `removeFavouriteRoom(int, int)`.
    pub fn remove_favourite_room(user_id: i32, room_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_room_favourites WHERE user_id = {user_id} AND room_id = {room_id}"
        ));
    }
}
