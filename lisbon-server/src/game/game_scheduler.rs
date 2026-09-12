//! Mirrors `net.h4bbo.lisbon.game.GameScheduler`.
//!
//! The Java `ScheduledExecutorService` is backed by dedicated blocking
//! threads: the 1 s fixed-rate tick loop runs on its own thread and
//! `schedule` (the Java one-shot `executor.schedule`) spawns a thread that
//! sleeps for the delay in milliseconds before running the closure once.

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

use parking_lot::{Mutex, MutexGuard};

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::catalogue::collectables::collectables_manager::CollectablesManager;
use crate::game::catalogue::rare_manager::RareManager;
use crate::game::entity::entity::Entity;
use crate::game::events::events_manager::EventsManager;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::moderation::chat_manager::ChatManager;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

/// The server-wide scheduler: the 1 s tick loop, the one-shot delayed
/// `schedule` and the credits / item-save / item-delete queues.
pub struct GameScheduler {
    tick_rate: AtomicI64,
    credits_handout_queue: Mutex<Vec<(Arc<Mutex<Player>>, i32)>>,
    item_saving_queue: Mutex<Vec<Item>>,
    item_deletion_queue: Mutex<Vec<i32>>,
}

impl GameScheduler {
    /// Mirrors `getInstance()`. The tick thread is started once, next to
    /// the Java constructor's `scheduleAtFixedRate(this, 0, 1, SECONDS)`.
    pub fn get_instance() -> &'static GameScheduler {
        static INSTANCE: OnceLock<GameScheduler> = OnceLock::new();
        static TICK_THREAD: OnceLock<()> = OnceLock::new();
        let instance = INSTANCE.get_or_init(|| GameScheduler {
            tick_rate: AtomicI64::new(0),
            credits_handout_queue: Mutex::new(Vec::new()),
            item_saving_queue: Mutex::new(Vec::new()),
            item_deletion_queue: Mutex::new(Vec::new()),
        });

        if TICK_THREAD.set(()).is_ok() {
            thread::spawn(move || loop {
                instance.tick();
                thread::sleep(Duration::from_secs(1));
            });
        }

        instance
    }

    /// Mirrors `run()` (the 1 s fixed-rate tick).
    fn tick(&self) {
        let tick = self.tick_rate.load(Ordering::SeqCst);

        let body = || -> Result<(), ()> {
            PlayerManager::get_instance().check_player_peak();

            for player_arc in PlayerManager::get_instance().get_players().iter() {
                let mut player = player_arc.lock();
                let Some(room_user) = player.get_room_user() else {
                    continue;
                };

                if room_user.get_room().is_none() {
                    continue;
                }

                // Port note: the Java sleep / AFK checks
                // (`getSleepTimer()` / `getAfkTimer()`) are not
                // ported; the Rust `RoomEntity` state has no AFK /
                // sleep timers.

                if GameConfiguration::get_instance().get_bool("credits.scheduler.enabled")
                    && i64::from(DateUtil::get_current_time_seconds())
                        > player.get_details().get_next_handout()
                {
                    if !room_user.contains_status(StatusType::AvatarSleep) {
                        let amount =
                            GameConfiguration::get_instance().get_integer("credits.scheduler.amount");
                        self.queue_player_credits(player_arc.clone(), amount);
                    }

                    player.get_details_mut().reset_next_handout();
                }
            }

            // Hand out queued credits every 30 seconds
            if GameConfiguration::get_instance().get_bool("credits.scheduler.enabled")
                && tick % 30 == 0
            {
                self.flush_credits_handout();
            }

            // Purge expired rows once a day
            if tick % 86_400 == 0 {
                EventsManager::get_instance().remove_expired_events();
            }

            // Item saving queue ticker every 10 seconds
            if tick % 10 == 0 {
                self.perform_item_saving();
            }

            // Item deletion queue ticker every 5 seconds
            if tick % 5 == 0 {
                self.perform_item_deletion();
            }

            // Delete expired CFH's every 60 seconds
            if tick % 60 == 0 {
                CallForHelpManager::get_instance().purge_expired_cfh();
            }

            // Save chat messages every 60 seconds
            if tick % 60 == 0 {
                ChatManager::get_instance().perform_chat_saving();
            }

            CollectablesManager::get_instance().check_expiries();
            RareManager::get_instance().perform_rare_manager_job(&self.tick_rate);

            Ok(())
        };

        if catch_unwind(AssertUnwindSafe(body)).is_err() {
            tracing::error!(target: "ErrorLogger", "GameScheduler crashed");
        }

        self.tick_rate.fetch_add(1, Ordering::SeqCst);
    }

    /// Mirrors the Java `LinkedHashMap<PlayerDetails, Integer>` handout
    /// flush: a second entry for the same player overwrites the amount,
    /// the bulk update runs, then every drained entry gets a
    /// `CREDIT_BALANCE`.
    fn flush_credits_handout(&self) {
        let handout = self.credits_handout_queue.lock().drain(..).collect::<Vec<_>>();
        if handout.is_empty() {
            return;
        }

        let mut guards: Vec<MutexGuard<'_, Player>> = Vec::new();
        let mut amounts: Vec<i32> = Vec::new();
        let mut last_index: HashMap<i32, usize> = HashMap::new();

        for (player_arc, amount) in &handout {
            let guard = player_arc.lock();
            let id = guard.get_details().get_id();

            if let Some(&index) = last_index.get(&id) {
                amounts[index] = *amount;
                continue;
            }

            last_index.insert(id, guards.len());
            guards.push(guard);
            amounts.push(*amount);
        }

        let entries: Vec<(&PlayerDetails, i32)> = guards
            .iter()
            .zip(amounts.iter())
            .map(|(guard, amount)| (guard.get_details(), *amount))
            .collect();

        CurrencyDao::increase_credits_bulk(&entries);

        for (player_arc, _) in &handout {
            let credits = player_arc.lock().get_details().get_credits();
            player_arc.lock().send(&CREDIT_BALANCE::new(credits));
        }
    }

    /// Mirrors `queuePlayerCredits(Player, int)`.
    pub fn queue_player_credits(&self, player: Arc<Mutex<Player>>, credits: i32) {
        self.credits_handout_queue.lock().push((player, credits));
    }

    /// Mirrors `queueSaveItem(Item)`.
    pub fn queue_save_item(&self, item: &Item) {
        let mut queue = self.item_saving_queue.lock();
        queue.retain(|queued| queued.get_id() != item.get_id());
        queue.push(item.clone());
    }

    /// Mirrors `queueDeleteItem(int)`.
    pub fn queue_delete_item(&self, item_id: i32) {
        let mut queue = self.item_deletion_queue.lock();
        queue.retain(|queued| *queued != item_id);
        queue.push(item_id);
    }

    /// Mirrors `performItemSaving()`.
    pub fn perform_item_saving(&self) {
        ItemManager::get_instance().perform_item_saving(&self.item_saving_queue);
    }

    /// Mirrors `performItemDeletion()`.
    pub fn perform_item_deletion(&self) {
        ItemManager::get_instance().perform_item_deletion(&self.item_deletion_queue);
    }

    /// Mirrors `getService().schedule(Runnable, long, TimeUnit)` (the
    /// one-shot delayed execution; the delay is in milliseconds).
    pub fn schedule<F: FnOnce() + Send + 'static>(&self, runnable: F, delay_ms: i64) {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(delay_ms.max(0) as u64));
            runnable();
        });
    }
}
