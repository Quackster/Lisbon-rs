//! Mirrors `net.h4bbo.lisbon.dao.mysql.RecyclerDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::recycler::recycler_reward::RecyclerReward;
use crate::game::recycler::recycler_session::RecyclerSession;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct RecyclerDao;

impl RecyclerDao {
    /// Mirrors `getRewards()`.
    pub fn get_rewards() -> Vec<RecyclerReward> {
        let mut recycler_reward_list = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM recycler_rewards ORDER BY id ASC") {
            if let (Some(id), Some(sale_code), Some(item_cost), Some(recycling_session_time_seconds), Some(collection_time_seconds)) = (
                row.i32("id"),
                row.str("sale_code"),
                row.i32("item_cost"),
                row.i32("recycling_session_time_seconds"),
                row.i32("collection_time_seconds"),
            ) {
                recycler_reward_list.push(RecyclerReward::new(
                    id,
                    sale_code,
                    item_cost,
                    recycling_session_time_seconds,
                    collection_time_seconds,
                ));
            }
        }

        recycler_reward_list
    }

    /// Mirrors `getSession(int)`.
    pub fn get_session(user_id: i32) -> Option<RecyclerSession> {
        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM recycler_sessions WHERE user_id = {user_id}"),
        ) {
            if let (Some(reward_id), Some(session_started), Some(items)) = (
                row.i32("reward_id"),
                row.i64("session_started"),
                row.str("items"),
            ) {
                return Some(RecyclerSession::new(reward_id, session_started, &items));
            }
        }

        None
    }

    /// Mirrors `createSession(int, int, String)`.
    pub fn create_session(user_id: i32, reward_id: i32, items: &str) -> RecyclerSession {
        Storage::get_storage().execute(&format!(
            "INSERT INTO recycler_sessions (user_id, reward_id, items) VALUES ({user_id}, {reward_id}, '{}')",
            escape(items)
        ));

        RecyclerSession::new(reward_id, DateUtil::get_current_time_seconds() as i64, items)
    }

    /// Mirrors `deleteSession(int)`.
    pub fn delete_session(user_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM recycler_sessions WHERE user_id = {user_id}"
        ));
    }
}
