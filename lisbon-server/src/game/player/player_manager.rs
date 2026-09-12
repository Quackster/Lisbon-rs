//! Mirrors `net.h4bbo.lisbon.game.player.PlayerManager`.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use parking_lot::{Mutex, MutexGuard};

use chrono::Timelike;

use crate::game::game_scheduler::GameScheduler;
use crate::game::room::enums::status_type::StatusType;
use crate::game::texts::texts_manager::TextsManager;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::messenger::messenger::Messenger;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::register::register_data_type::RegisterDataType;
use crate::game::player::register::register_value::RegisterValue;
use crate::lisbon::Lisbon;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::openinghours::info_hotel_closing::INFO_HOTEL_CLOSING;
use crate::messages::outgoing::openinghours::info_hotel_closed::INFO_HOTEL_CLOSED;
use crate::messages::types::MessageComposer;
use crate::util::date_util::DateUtil;

/// Mirrors the `Object` returned by `getRegisterValue` (the `String` value
/// for `STRING` fields, the boolean `flag` otherwise).
#[derive(Clone, Debug)]
pub enum RegisterValueData {
    String(String),
    Boolean(bool),
}

/// Mirrors `PlayerManager`.
///
/// Java is a static singleton backed by a `CopyOnWriteArrayList` of shared
/// `Player` references; here players are `Arc<Mutex<Player>>` so every holder
/// (the manager list and any lookup result) sees the same object, and the
/// global `&'static` instance keeps its state behind `parking_lot` locks so
/// all methods take `&self`.
///
/// Port note: the Java `shutdownTimeout` (`ScheduledFuture<?>`) is
/// represented by the `maintenance_generation` counter plus a one-shot
/// `GameScheduler::schedule`; bumping the generation cancels the pending
/// shutdown (the `ScheduledFuture.cancel(true)` equivalent).
pub struct PlayerManager {
    players: Mutex<Vec<Arc<Mutex<Player>>>>,
    time_until_next_reset: Mutex<i64>,
    daily_player_peak: Mutex<i64>,
    is_maintenance_shutdown: Mutex<bool>,
    maintenance_at: Mutex<Option<Duration>>,
    maintenance_generation: Arc<AtomicI64>,
}

impl PlayerManager {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            players: Mutex::new(Vec::new()),
            time_until_next_reset: Mutex::new(0),
            daily_player_peak: Mutex::new(0),
            is_maintenance_shutdown: Mutex::new(false),
            maintenance_at: Mutex::new(None),
            maintenance_generation: Arc::new(AtomicI64::new(0)),
        }
    }

    /// Checks and sets the daily player peak.
    pub fn check_player_peak(&self) {
        let now = DateUtil::get_current_time_seconds() as i64;

        if now > *self.time_until_next_reset.lock() {
            *self.time_until_next_reset.lock() = now + 86_400; // `TimeUnit.DAYS.toSeconds(1)`
            *self.daily_player_peak.lock() = self.get_players().len() as i64;
        } else {
            let new_size = self.get_players().len() as i64;

            let mut peak = self.daily_player_peak.lock();
            if new_size > *peak {
                *peak = new_size;
            }
        }
    }

    /// Get a player by user id.
    ///
    /// Returns a shared handle; the manager lock is released before returning.
    pub fn get_player_by_id(&self, user_id: i32) -> Option<Arc<Mutex<Player>>> {
        let players = self.players.lock();
        players
            .iter()
            .find(|player| player.lock().get_details().get_id() == user_id)
            .cloned()
    }

    /// Get the player by name.
    ///
    /// Returns a shared handle; the manager lock is released before returning.
    pub fn get_player_by_name(&self, username: &str) -> Option<Arc<Mutex<Player>>> {
        let players = self.players.lock();
        players
            .iter()
            .find(|player| {
                player
                    .lock()
                    .get_details()
                    .get_name()
                    .eq_ignore_ascii_case(username)
            })
            .cloned()
    }

    /// Get a player data by user id (offline players fall back to the
    /// database).
    pub fn get_player_data_by_id(&self, user_id: i32) -> Option<PlayerDetails> {
        if let Some(player) = self.get_player_by_id(user_id) {
            return Some(player.lock().get_details().clone());
        }

        PlayerDao::get_details(user_id)
    }

    /// Get a player data by username (offline players fall back to the
    /// database).
    pub fn get_player_data_by_name(&self, username: &str) -> Option<PlayerDetails> {
        if let Some(player) = self.get_player_by_name(username) {
            return Some(player.lock().get_details().clone());
        }

        PlayerDao::get_details_by_name(username)
    }

    /// Mirrors `getMessengerData(int)`.
    pub fn get_messenger_data_by_id(&self, user_id: i32) -> Option<Messenger> {
        if let Some(player) = self.get_player_by_id(user_id) {
            return player.lock().get_messenger().cloned();
        }

        self.get_player_data_by_id(user_id).map(|details| Messenger::from_details(&details))
    }

    /// Mirrors `getMessengerData(String)`.
    pub fn get_messenger_data_by_name(&self, username: &str) -> Option<Messenger> {
        if let Some(player) = self.get_player_by_name(username) {
            return player.lock().get_messenger().cloned();
        }

        self.get_player_data_by_name(username).map(|details| Messenger::from_details(&details))
    }

    /// Remove a player from the list, this is handled automatically when
    /// the socket is closed.
    pub fn remove_player(&self, player: &Arc<Mutex<Player>>) {
        // The Java `name == null` guard is inapplicable (Rust `&str`).
        // Identity removal, like Java's `ArrayList.remove(Object)`.
        self.players.lock().retain(|p| !Arc::ptr_eq(p, player));
    }

    /// Add a player to the list, this is handled automatically when
    /// the player is logged in.
    pub fn add_player(&self, player: Arc<Mutex<Player>>) {
        // The Java `details == null` guard is inapplicable.
        self.players.lock().push(player);
    }

    /// Disconnect a session by user id (the matches are collected first so
    /// the `kickFromServer` -> `dispose` -> `removePlayer` re-entrance does
    /// not deadlock on the players lock, like the Java
    /// `CopyOnWriteArrayList` would not).
    pub fn disconnect_session(&self, user_id: i32) {
        let targets: Vec<Arc<Mutex<Player>>> = self
            .players
            .lock()
            .iter()
            .filter(|player| player.lock().get_details().get_id() == user_id)
            .cloned()
            .collect();

        for player in targets {
            player.lock().kick_from_server();
        }
    }

    /// Start shutdown timeout.
    pub fn plan_maintenance(&self, maintenance_at: Duration) {
        // Interrupt current timeout to set new maintenance countdown.
        let generation = self
            .maintenance_generation
            .fetch_add(1, Ordering::SeqCst)
            + 1;
        let counter = Arc::clone(&self.maintenance_generation);
        let delay_ms = maintenance_at.as_secs() as i64 * 1000
            + maintenance_at.subsec_millis() as i64;
        GameScheduler::get_instance().schedule(
            move || {
                // `System.exit(0)`, invalidated when the generation moved
                // (cancel or a new plan).
                if counter.load(Ordering::SeqCst) == generation {
                    std::process::exit(0);
                }
            },
            delay_ms,
        );

        *self.is_maintenance_shutdown.lock() = true;
        *self.maintenance_at.lock() = Some(maintenance_at);

        // Notify all users of shutdown timeout.
        let players = self.players.lock();
        for player in players.iter() {
            player.lock().send(&INFO_HOTEL_CLOSING::new(maintenance_at));
        }
    }

    /// Cancel shutdown timeout.
    pub fn cancel_maintenance(&self) {
        // Cancel current timeout.
        self.maintenance_generation.fetch_add(1, Ordering::SeqCst);

        *self.is_maintenance_shutdown.lock() = false;

        // Notify all users maintenance has been cancelled.
        let players = self.players.lock();
        for player in players.iter() {
            player.lock().send(&ALERT::new(&TextsManager::get_instance().get_value("maintenance_cancelled")));
        }
    }

    /// Send a message to all users.
    pub fn send_all(&self, composer: &dyn MessageComposer) {
        let players = self.players.lock();
        for player in players.iter() {
            player.lock().send(composer);
        }
    }

    /// Close and dispose all users.
    pub fn dispose(&self) {
        let players = self.players.lock();
        for player in players.iter() {
            let mut player = player.lock();
            // Send fancy maintenance alert if we're shutting down.
            let now = chrono::Local::now();
            player.send(&INFO_HOTEL_CLOSED::new(now.hour() as i32, now.minute() as i32, false));
            // Now disconnect the player.
            player.kick_from_server();
        }
    }

    /// Get the collection of players on the server.
    ///
    /// The Java `CopyOnWriteArrayList` is exposed as a `parking_lot`-guarded
    /// `Vec` of shared handles; callers take the guard for the duration of
    /// their access.
    pub fn get_players(&self) -> MutexGuard<'_, Vec<Arc<Mutex<Player>>>> {
        self.players.lock()
    }

    /// Get the collection of active players on the server.
    pub fn get_active_players(&self) -> Vec<Arc<Mutex<Player>>> {
        let mut active_players = Vec::new();
        let players = self.players.lock();
        for player in players.iter() {
            let guard = player.lock();
            let Some(room_user) = guard.get_room_user() else {
                continue;
            };
            if room_user.get_room().is_none() {
                continue;
            }

            if room_user.contains_status(StatusType::AvatarSleep) {
                continue;
            }

            active_players.push(player.clone());
        }

        active_players
    }

    /// Create password hash.
    pub fn create_password(&self, password: &str) -> String {
        Lisbon::get_password_encoder().encode(password)
    }

    /// Get whether the hash matches the entered password.
    pub fn password_matches(&self, database_password: &str, entered_password: &str) -> bool {
        Lisbon::get_password_encoder().matches(entered_password, database_password)
    }

    /// Get values for registering.
    ///
    /// Java `LinkedHashMap` → `BTreeMap` (insertion order is key order,
    /// which `BTreeMap` preserves).
    pub fn get_register_values(&self) -> BTreeMap<i32, RegisterValue> {
        let mut register_values = BTreeMap::new();
        register_values.insert(1, RegisterValue::new("parentagree", RegisterDataType::Boolean));
        register_values.insert(2, RegisterValue::new("name", RegisterDataType::String));
        register_values.insert(3, RegisterValue::new("password", RegisterDataType::String));
        register_values.insert(4, RegisterValue::new("figure", RegisterDataType::String));
        register_values.insert(5, RegisterValue::new("sex", RegisterDataType::String));
        register_values.insert(6, RegisterValue::new("customData", RegisterDataType::String));
        register_values.insert(7, RegisterValue::new("email", RegisterDataType::String));
        register_values.insert(8, RegisterValue::new("birthday", RegisterDataType::String));
        register_values.insert(9, RegisterValue::new("directMail", RegisterDataType::Boolean));
        register_values.insert(10, RegisterValue::new("has_read_agreement", RegisterDataType::Boolean));
        register_values.insert(11, RegisterValue::new("isp_id", RegisterDataType::String));
        register_values.insert(12, RegisterValue::new("partnersite", RegisterDataType::String));
        register_values.insert(13, RegisterValue::new("oldpassword", RegisterDataType::String));
        register_values
    }

    /// Get if the player is online.
    pub fn is_player_online(&self, user_id: i32) -> bool {
        let players = self.players.lock();
        for player in players.iter() {
            let player = player.lock();
            if player.get_details().get_id() != user_id {
                continue;
            }

            return player.get_details().is_online_status_visible();
        }

        false
    }

    /// Get the value for a register label.
    pub fn get_register_value(
        &self,
        values: &BTreeMap<i32, RegisterValue>,
        label: &str,
    ) -> Option<RegisterValueData> {
        for value in values.values() {
            if value.get_label() == label {
                return Some(if value.get_data_type() == RegisterDataType::String {
                    RegisterValueData::String(value.get_value().to_string())
                } else {
                    RegisterValueData::Boolean(value.get_flag())
                });
            }
        }

        None
    }

    /// Get daily player peak.
    pub fn get_daily_player_peak(&self) -> i64 {
        *self.daily_player_peak.lock()
    }

    /// Get duration until shutdown.
    pub fn get_maintenance_at(&self) -> Option<Duration> {
        *self.maintenance_at.lock()
    }

    /// Get maintenance shutdown status.
    pub fn is_maintenance(&self) -> bool {
        *self.is_maintenance_shutdown.lock()
    }

    /// Gets the instance.
    pub fn get_instance() -> &'static PlayerManager {
        static INSTANCE: OnceLock<PlayerManager> = OnceLock::new();
        INSTANCE.get_or_init(PlayerManager::new)
    }
}
