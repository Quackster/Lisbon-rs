//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomVisitsDao`.

use crate::dao::storage::{RowGetters, Storage};

pub struct RoomVisitsDao;

impl RoomVisitsDao {
    /// Mirrors `addVisit(int, int)`.
    pub fn add_visit(user_id: i32, room_id: i32) {
        Storage::get_storage().execute(&format!(
            "REPLACE INTO room_visits (user_id, room_id, visited_at) VALUES ({user_id}, {room_id}, NOW())"
        ));
    }

    /// Mirrors `countVisits(int)`.
    pub fn count_visits(user_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!("SELECT COUNT(*) as visits FROM room_visits WHERE user_id = {user_id}"),
        ) {
            if let Some(visits) = row.i32("visits") {
                count = visits;
            }
        }

        count
    }
}
