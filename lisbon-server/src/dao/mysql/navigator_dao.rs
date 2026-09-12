//! Mirrors `net.h4bbo.lisbon.dao.mysql.NavigatorDao`.

use std::collections::HashMap;

use crate::dao::mysql::room_dao::RoomDao;
use crate::dao::storage::{RowGetters, Storage};
use crate::game::navigator::navigator_category::NavigatorCategory;
use crate::game::player::player_rank::PlayerRank;
use crate::game::room::room::Room;

pub struct NavigatorDao;

impl NavigatorDao {
    /// Mirrors `getCategories()`.
    pub fn get_categories() -> HashMap<i32, NavigatorCategory> {
        let mut categories: HashMap<i32, NavigatorCategory> = HashMap::new();

        for row in Storage::get_storage().query_all(
            "SELECT * FROM rooms_categories ORDER BY order_id ASC",
        ) {
            if let Some(id) = row.i32("id") {
                let category = NavigatorCategory::new(
                    id,
                    row.i32("parent_id").unwrap_or(0),
                    row.str("name").unwrap_or_default(),
                    row.bool("public_spaces").unwrap_or(false),
                    row.bool("allow_trading").unwrap_or(false),
                    PlayerRank::get_rank_for_id(row.i32("minrole_access").unwrap_or(0))
                        .unwrap_or(PlayerRank::Rankless),
                    PlayerRank::get_rank_for_id(row.i32("minrole_setflatcat").unwrap_or(0))
                        .unwrap_or(PlayerRank::Rankless),
                    row.bool("isnode").unwrap_or(false),
                );

                categories.insert(category.get_id(), category);
            }
        }

        categories
    }

    /// Mirrors `getRecentRooms(int, int)`.
    pub fn get_recent_rooms(limit: i32, category_id: i32) -> Vec<Room> {
        let mut rooms = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM rooms LEFT JOIN users ON rooms.owner_id = users.id WHERE category = {category_id} AND owner_id > 0 ORDER BY visitors_now DESC, rooms.id DESC LIMIT {limit}"
            ),
        ) {
            rooms.push(RoomDao::fill_room(&row));
        }

        rooms
    }

    /// Mirrors `createRoom(int, String, String, boolean, int)`.
    pub fn create_room(
        owner_id: i32,
        room_name: &str,
        room_model: &str,
        room_show_name: bool,
        access_type: i32,
    ) -> i32 {
        match Storage::get_storage().execute_insert(&format!(
            "INSERT INTO rooms (owner_id, name, description, model, showname, password, accesstype) VALUES ({owner_id}, '{}', '', '{}', {}, '', {access_type})",
            room_name.replace('\'', "''"),
            room_model.replace('\'', "''"),
            if room_show_name { 1 } else { 0 }
        )) {
            Some(id) => id as i32,
            None => 0,
        }
    }
}
