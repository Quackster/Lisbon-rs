//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomRightsDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::player::player_details::PlayerDetails;
use crate::game::room::room_data::RoomData;

pub struct RoomRightsDao;

impl RoomRightsDao {
    /// Mirrors `getRoomRights(RoomData)`.
    pub fn get_room_rights(room: &RoomData) -> Vec<i32> {
        let mut users = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT user_id FROM rooms_rights WHERE room_id = {}", room.get_id()))
        {
            if let Some(user_id) = row.i32("user_id") {
                users.push(user_id);
            }
        }

        users
    }

    /// Mirrors `addRights(PlayerDetails, RoomData)`.
    pub fn add_rights(user: &PlayerDetails, room: &RoomData) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO rooms_rights (user_id, room_id) VALUES ({}, {})",
            user.get_id(),
            room.get_id()
        ));
    }

    /// Mirrors `removeRights(PlayerDetails, RoomData)`.
    pub fn remove_rights(user: &PlayerDetails, room: &RoomData) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM rooms_rights WHERE user_id = {} AND room_id = {}",
            user.get_id(),
            room.get_id()
        ));
    }

    /// Mirrors `deleteRoomRights(RoomData)`.
    pub fn delete_room_rights(room: &RoomData) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM rooms_rights WHERE room_id = {}",
            room.get_id()
        ));
    }
}
