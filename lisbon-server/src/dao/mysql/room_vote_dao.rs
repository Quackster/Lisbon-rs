//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomVoteDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::util::date_util::DateUtil;

const EXPIRE_SECONDS: i64 = 30 * 86_400;

pub struct RoomVoteDao;

impl RoomVoteDao {
    /// Mirrors `vote(int, int, int)`.
    pub fn vote(user_id: i32, room_id: i32, answer: i32) {
        let expire_time = DateUtil::get_current_time_seconds() as i64 + EXPIRE_SECONDS;

        Storage::get_storage().execute(&format!(
            "INSERT INTO users_room_votes (user_id, room_id, vote, expire_time) VALUES ({user_id}, {room_id}, {answer}, {expire_time})"
        ));
    }

    /// Mirrors `removeExpiredVotes(int)`.
    pub fn remove_expired_votes(room_id: i32) {
        let now = DateUtil::get_current_time_seconds() as i64;

        Storage::get_storage().execute(&format!(
            "DELETE FROM users_room_votes WHERE room_id = {room_id} AND expire_time < {now}"
        ));
    }

    /// Mirrors `getRatings(int)`.
    pub fn get_ratings(room_id: i32) -> HashMap<i32, i32> {
        let mut ratings = HashMap::new();
        let now = DateUtil::get_current_time_seconds() as i64;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT user_id,vote FROM users_room_votes WHERE room_id = {room_id} AND expire_time > {now}"
            ),
        ) {
            if let (Some(user_id), Some(vote)) = (row.i32("user_id"), row.i32("vote")) {
                ratings.insert(user_id, vote);
            }
        }

        ratings
    }
}
