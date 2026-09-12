//! Mirrors `net.h4bbo.lisbon.game.commands.registered.UptimeCommand`.

use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::lisbon::Lisbon;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::date_util::DateUtil;

static MEMORY_USAGE: AtomicI32 = AtomicI32::new(0);
static ACTIVE_PLAYERS: AtomicI32 = AtomicI32::new(0);
static AUTHENTICATED_PLAYERS: AtomicI32 = AtomicI32::new(0);

const UPTIME_COMMAND_INTERVAL_SECONDS: i64 = 5;
const CPU_ARCHITECTURE: &'static str = std::env::consts::ARCH;
const JVM_NAME: &'static str = "N/A (Rust)";
const OPERATING_SYSTEM_NAME: &'static str = std::env::consts::OS;

static CPU_NUM_THREADS: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

fn cpu_num_threads() -> i32 {
    *CPU_NUM_THREADS.get_or_init(|| {
        std::thread::available_parallelism()
            .map(|n| n.get() as i32)
            .unwrap_or(0)
    })
}

/// Mirrors `UptimeCommand`.
pub struct UptimeCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
    expiry: AtomicI64,
}

impl Command for UptimeCommand {
    /// Mirrors the no-arg constructor.
    fn new() -> Self {
        let mut this = Self {
            permissions: Vec::new(),
            arguments: Vec::new(),
            expiry: AtomicI64::new(DateUtil::get_current_time_seconds() as i64),
        };
        this.add_permissions();
        this.add_arguments();
        this
    }

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self) {
        self.permissions.push(Fuseright::Default);
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, _args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        if DateUtil::get_current_time_seconds() as i64 > self.expiry.load(Ordering::Relaxed) {
            AUTHENTICATED_PLAYERS.store(
                PlayerManager::get_instance().get_players().len() as i32,
                Ordering::Relaxed,
            );
            ACTIVE_PLAYERS.store(
                PlayerManager::get_instance().get_active_players().len() as i32,
                Ordering::Relaxed,
            );

            // Port note: no equivalent of `Runtime.totalMemory()` /
            // `Runtime.freeMemory()` in Rust; left at zero.
            MEMORY_USAGE.store(0, Ordering::Relaxed);

            self.expiry.store(
                DateUtil::get_current_time_seconds() as i64 + UPTIME_COMMAND_INTERVAL_SECONDS,
                Ordering::Relaxed,
            );
        }

        // Note: the Java source mixes seconds and milliseconds
        // (`(getCurrentTimeSeconds() - getStartupTime()) * 1000`); mirrored verbatim.
        let uptime =
            (DateUtil::get_current_time_seconds() as i64 - Lisbon::get_startup_time()) * 1000;
        let days = uptime / (1000 * 60 * 60 * 24);
        let hours = (uptime - days * (1000 * 60 * 60 * 24)) / (1000 * 60 * 60);
        let minutes =
            (uptime - days * (1000 * 60 * 60 * 24) - hours * (1000 * 60 * 60)) / (1000 * 60);
        let seconds = (uptime
            - days * (1000 * 60 * 60 * 24)
            - hours * (1000 * 60 * 60)
            - minutes * (1000 * 60))
            / 1000;

        let active_players = ACTIVE_PLAYERS.load(Ordering::Relaxed);
        let authenticated_players = AUTHENTICATED_PLAYERS.load(Ordering::Relaxed);
        let memory_usage = MEMORY_USAGE.load(Ordering::Relaxed);

        let alert_message = format!(
            "SERVER\rServer uptime is {} day(s), {} hour(s), {} minute(s) and {} second(s)<br>\
             There are {} active players, and {} authenticated players<br>\
             Daily player peak count: {}<br>\
             <br>\
             SYSTEM<br>\
             CPU architecture: {}<br>\
             CPU cores: {}<br>\
             memory usage: {} MB<br>\
             JVM: {}<br>\
             OS: {}",
            days,
            hours,
            minutes,
            seconds,
            active_players,
            authenticated_players,
            PlayerManager::get_instance().get_daily_player_peak(),
            CPU_ARCHITECTURE,
            cpu_num_threads(),
            memory_usage,
            JVM_NAME,
            OPERATING_SYSTEM_NAME
        );

        player.send(&ALERT::new(&alert_message));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Get the uptime and status of the server".to_string()
    }

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright> {
        self.permissions.clone()
    }

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}
