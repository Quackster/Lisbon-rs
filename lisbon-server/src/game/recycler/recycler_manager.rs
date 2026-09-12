//! Mirrors `net.h4bbo.lisbon.game.recycler.RecyclerManager`.

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::recycler_dao::RecyclerDao;
use crate::game::recycler::recycler_reward::RecyclerReward;
use crate::log::Log;
use crate::util::config::game_configuration::GameConfiguration;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<RecyclerManager>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct RecyclerManager {
    recycler_rewards: Vec<RecyclerReward>,
    recycler_timeout_seconds: i32,
    recycler_session_length_seconds: i32,
    recycler_item_quarantine_seconds: i32,
    recycler_enabled: bool,
}

impl RecyclerManager {
    fn new() -> Self {
        let recycler_timeout_seconds = GameConfiguration::get_instance()
            .get_integer("recycler.max.time.to.collect.seconds");
        let recycler_session_length_seconds = GameConfiguration::get_instance()
            .get_integer("recycler.session.length.seconds");
        let recycler_item_quarantine_seconds = GameConfiguration::get_instance()
            .get_integer("recycler.item.quarantine.seconds");

        let mut recycler_enabled = true;
        let mut recycler_rewards = RecyclerDao::get_rewards();
        for recycler_reward in &recycler_rewards {
            if recycler_reward.get_catalogue_item().is_none() {
                Log::get_error_logger()
                    .error(format!(
                        "Could not locate catalogue item with sale code: {}",
                        recycler_reward.get_sale_code()
                    ));
                recycler_enabled = false;
            }
        }

        if recycler_rewards.is_empty() {
            recycler_enabled = false;
        }

        if !recycler_enabled {
            recycler_rewards.clear();
            tracing::warn!("Recycler is disabled");
        }

        Self {
            recycler_rewards,
            recycler_timeout_seconds,
            recycler_session_length_seconds,
            recycler_item_quarantine_seconds,
            recycler_enabled,
        }
    }

    /// Mirrors `isRecyclerEnabled()`.
    pub fn is_recycler_enabled(&self) -> bool {
        self.recycler_enabled
    }

    /// Mirrors `getRecyclerRewards()`.
    pub fn get_recycler_rewards(&self) -> Vec<RecyclerReward> {
        self.recycler_rewards.clone()
    }

    /// Mirrors `getRecyclerTimeoutSeconds()`.
    pub fn get_recycler_timeout_seconds(&self) -> i32 {
        self.recycler_timeout_seconds
    }

    /// Mirrors `getRecyclerSessionLengthSeconds()`.
    pub fn get_recycler_session_length_seconds(&self) -> i32 {
        self.recycler_session_length_seconds
    }

    /// Mirrors `getRecyclerItemQuarantineSeconds()`.
    pub fn get_recycler_item_quarantine_seconds(&self) -> i32 {
        self.recycler_item_quarantine_seconds
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<RecyclerManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
