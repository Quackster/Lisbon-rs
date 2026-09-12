//! Mirrors `net.h4bbo.lisbon.game.recycler.RecyclerSession`.

use crate::game::recycler::recycler_manager::RecyclerManager;
use crate::game::recycler::recycler_reward::RecyclerReward;
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug)]
pub struct RecyclerSession {
    reward_id: i32,
    session_started: i64,
    items: Vec<i32>,
}

impl RecyclerSession {
    /// Mirrors the 3-arg `RecyclerSession(int, long, String)` constructor.
    pub fn new(reward_id: i32, session_started: i64, items: &str) -> Self {
        let items = items
            .split(',')
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();
        Self {
            reward_id,
            session_started,
            items,
        }
    }

    /// Mirrors `getMinutesPassed()`.
    pub fn get_minutes_passed(&self) -> i32 {
        let seconds = DateUtil::get_current_time_seconds() as i64 - self.session_started;
        (seconds / 60) as i32
    }

    /// Mirrors `getMinutesLeft()`.
    pub fn get_minutes_left(&self) -> i32 {
        if !self.is_recycling_done() {
            if let Some(reward) = self.get_recycler_reward() {
                let seconds =
                    reward.get_recycling_time_sessions() as i64
                        - (DateUtil::get_current_time_seconds() as i64 - self.session_started);
                return (seconds / 60) as i32;
            }
        }

        0
    }

    /// Mirrors `isRecyclingDone()`.
    pub fn is_recycling_done(&self) -> bool {
        match self.get_recycler_reward() {
            Some(reward) => {
                let recycling_minutes = reward.get_recycling_time_sessions() as i64 / 60;
                self.get_minutes_passed() >= recycling_minutes as i32
            }
            None => false,
        }
    }

    /// Mirrors `hasTimeout()`.
    pub fn has_timeout(&self) -> bool {
        if !self.is_recycling_done() {
            return false;
        }

        match self.get_recycler_reward() {
            Some(reward) => {
                let total_minutes =
                    (reward.get_recycling_time_sessions() + reward.get_collection_time_seconds())
                        as i64
                        / 60;
                self.get_minutes_passed() >= total_minutes as i32
            }
            None => false,
        }
    }

    /// Mirrors `getRewardId()`.
    pub fn get_reward_id(&self) -> i32 {
        self.reward_id
    }

    /// Mirrors `getRecyclerReward()`.
    pub fn get_recycler_reward(&self) -> Option<RecyclerReward> {
        RecyclerManager::get_instance()
            .get_recycler_rewards()
            .into_iter()
            .find(|reward| reward.get_id() == self.reward_id)
    }

    /// Mirrors `getSessionStarted()`.
    pub fn get_session_started(&self) -> i64 {
        self.session_started
    }

    /// Mirrors `getItems()`.
    pub fn get_items(&self) -> Vec<i32> {
        self.items.clone()
    }
}
