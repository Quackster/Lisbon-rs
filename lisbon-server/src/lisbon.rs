//! Mirrors `net.h4bbo.lisbon.Lisbon` (the server entry point, ported in
//! Phase 5).
//!
//! Java is a class of `static` fields and `static` methods. Here the
//! singletons (`server`, `musServer`, `rconServer`) are backed by the
//! corresponding `*Server::get_instance()` accessors, and the remaining static
//! state (IP/port, startup time, the shutdown flag) is backed by process-wide
//! statics. `main(String[])` becomes the synchronous `run`, which does the
//! blocking bootstrap (config, storage, managers) and then drives the async
//! network servers on a tokio runtime.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::OnceLock;

use chrono::Datelike;
use parking_lot::RwLock;
use rand::Rng;

use argon2::password_hash::{PasswordHash, PasswordHasher, SaltString};

use crate::dao::mysql::settings_dao::SettingsDao;
use crate::dao::storage::Storage;
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::ads::ad_manager::AdManager;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::catalogue::collectables::collectables_manager::CollectablesManager;
use crate::game::commands::command_manager::CommandManager;
use crate::game::events::events_manager::EventsManager;
use crate::game::fuserights::fuserights_manager::FuserightsManager;
use crate::game::games::game_manager::GameManager;
use crate::game::games::snowstorm::snowstorm_maps_manager::SnowStormMapsManager;
use crate::game::game_scheduler::GameScheduler;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::item::item_manager::ItemManager;
use crate::game::misc::figure::figure_manager::FigureManager;
use crate::game::moderation::chat_manager::ChatManager;
use crate::game::navigator::navigator_manager::NavigatorManager;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;
use crate::game::room::models::room_model_manager::RoomModelManager;
use crate::game::room::room_manager::RoomManager;
use crate::game::player::player_manager::PlayerManager;
use crate::game::texts::texts_manager::TextsManager;
use crate::game::wordfilter::wordfilter_manager::WordfilterManager;
use crate::messages::message_handler::MessageHandler;
use crate::server::mus::mus_server::MusServer;
use crate::server::netty::netty_server::NettyServer;
use crate::server::rcon::rcon_server::RconServer;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::config::server_configuration::ServerConfiguration;
use crate::util::config::writer::{DefaultConfigWriter, GameConfigWriter};
use crate::util::date_util::DateUtil;

const DIST_CONFIG_PATH: &str = "config/server.ini";
const LEGACY_CONFIG_PATH: &str = "server.ini";

static SHUTTING_DOWN: OnceLock<AtomicBool> = OnceLock::new();
static STARTUP_TIME: OnceLock<i64> = OnceLock::new();
static SERVER_IP: RwLock<String> = RwLock::new(String::new());
static SERVER_PORT: AtomicI32 = AtomicI32::new(0);
static MUS_SERVER_IP: RwLock<String> = RwLock::new(String::new());
static MUS_SERVER_PORT: AtomicI32 = AtomicI32::new(0);
static RCON_IP: RwLock<String> = RwLock::new(String::new());
static RCON_PORT: AtomicI32 = AtomicI32::new(0);

/// Mirrors `Lisbon.PasswordEncoder` (Spring `Argon2PasswordEncoder` in Java).
///
/// Mirrors `new Argon2PasswordEncoder(16, 32, 1, 65536, 2)`: a 32-byte
/// random salt, a 32-byte Argon2id hash with 65536 KiB of memory
/// (`Params` log-memory = 16), 2 iterations and parallelism 1.
pub struct PasswordEncoder;

impl PasswordEncoder {
    fn encoder() -> argon2::Argon2<'static> {
        let params = argon2::Params::new(65_536, 2, 1, None)
            .expect("invalid Argon2 parameters");

        argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        )
    }

    /// Mirrors `encode`.
    pub fn encode(&self, s: &str) -> String {
        let mut salt_bytes = [0u8; 32];
        rand::thread_rng().fill(&mut salt_bytes[..]);
        use base64::Engine;
        let salt = SaltString::from_b64(
            &base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt_bytes),
        )
        .expect("valid salt bytes");

        Self::encoder()
            .hash_password(s.as_bytes(), &salt)
            .expect("Argon2 hashing failed")
            .to_string()
    }

    /// Mirrors `matches`.
    ///
    /// A stored value that is not a PHC string (legacy data) falls back to
    /// a plain comparison.
    pub fn matches(&self, a: &str, b: &str) -> bool {
        let stored = match PasswordHash::new(b) {
            Ok(stored) => stored,
            Err(_) => return a == b,
        };

        let params = match argon2::Params::try_from(&stored) {
            Ok(params) => params,
            Err(_) => return false,
        };
        let salt = match stored.salt {
            Some(salt) => salt,
            None => return false,
        };
        let expected = match stored.hash {
            Some(hash) => hash,
            None => return false,
        };

        match Self::encoder().hash_password_customized(a.as_bytes(), None, None, params, salt) {
            Ok(computed) => computed
                .hash
                .as_ref()
                .is_some_and(|actual| actual.as_bytes() == expected.as_bytes()),
            Err(_) => false,
        }
    }
}

/// Mirrors the `Lisbon.gson` field (`Gson`).
pub struct Gson;

impl Gson {
    /// Mirrors `Gson.toJson(Object)`.
    pub fn to_json<T: serde::Serialize>(&self, data: &T) -> String {
        serde_json::to_string(data).unwrap_or_default()
    }

    /// Mirrors `Gson.fromJson(String, Class<T>)` (`null` when the JSON does
    /// not parse).
    pub fn from_json<T: serde::de::DeserializeOwned>(&self, json: &str) -> Option<T> {
        serde_json::from_str(json).ok()
    }
}

/// Mirrors `Lisbon`.
pub struct Lisbon;

impl Lisbon {
    /// Mirrors `main(String[])` — the full bootstrap.
    ///
    /// The blocking part (config, storage, managers) runs on the caller thread;
    /// the async network servers are then driven on a fresh tokio runtime.
    pub fn run() {
        Self::get_startup_time();

        ServerConfiguration::set_writer(Box::new(DefaultConfigWriter::new()));
        ServerConfiguration::load(&Self::resolve_server_config_path());

        // Mirrors `ResourceLeakDetector.setLevel(ADVANCED)` (Netty-only, no-op
        // here).
        tracing::info!("Lisbon - Habbo Hotel V26 Emulation");

        if !Storage::connect() {
            return;
        }

        tracing::info!("Setting up game");

        GameConfiguration::get_instance_with_writer(GameConfigWriter::new());

        InfobusManager::get_instance();
        AchievementManager::get_instance();
        AdManager::get_instance();
        WalkwaysManager::get_instance();
        ItemManager::get_instance();
        CatalogueManager::get_instance();
        RoomModelManager::get_instance();
        RoomManager::get_instance();
        PlayerManager::get_instance();
        FuserightsManager::get_instance();
        NavigatorManager::get_instance();
        EventsManager::get_instance();
        ChatManager::get_instance();
        GameScheduler::get_instance();
        SnowStormMapsManager::get_instance();
        GameManager::get_instance();
        CommandManager::get_instance();
        MessageHandler::get_instance();
        TextsManager::get_instance();
        WordfilterManager::get_instance();
        CollectablesManager::get_instance();
        FigureManager::get_instance();

        // Update players online back to 0
        SettingsDao::update_setting("players.online", "0");

        Self::setup_mus();
        Self::setup_rcon();
        Self::setup_server();

        // Mirrors `Runtime.getRuntime().addShutdownHook(Lisbon::dispose)`; the
        // runtime is dropped when `block_on` returns.
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime");

        runtime.block_on(Self::start_servers());
    }

    /// Mirrors the `setup*.bind()` Netty event-loop startup.
    ///
    /// Spawns the three network servers as long-lived tasks and blocks the
    /// runtime until the process is signalled to stop.
    async fn start_servers() {
        tokio::spawn(NettyServer::get_instance().start());
        tokio::spawn(MusServer::get_instance().start());
        tokio::spawn(RconServer::get_instance().start());

        // The Java shutdown hook fires on both SIGINT and SIGTERM, so a plain
        // `kill <pid>` (SIGTERM) must also trigger a clean shutdown, not just
        // Ctrl-C. Register the SIGTERM handler and select on it alongside
        // `ctrl_c()`.
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to register the SIGTERM handler");

        // Keep the runtime alive (mirrors the non-daemon Netty event loops).
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                Self::shutdown().await;
            }
            _ = sigterm.recv() => {
                Self::shutdown().await;
            }
            _ = std::future::pending::<()>() => {}
        }
    }

    /// Runs `dispose` off the async context.
    ///
    /// `dispose` performs synchronous DB work (`block_on` on the storage
    /// runtime), which panics when the current thread is an async worker, so
    /// it is dispatched onto the blocking pool.
    async fn shutdown() {
        let _ = tokio::task::spawn_blocking(|| Self::dispose()).await;
    }

    /// Mirrors `resolveServerConfigPath`.
    fn resolve_server_config_path() -> String {
        let mut configured_path = std::env::var("LISBON_SERVER_CONFIG").unwrap_or_default();

        if configured_path.trim().is_empty() {
            configured_path = std::env::var("LISBON_CONFIG").unwrap_or_default();
        }

        if !configured_path.trim().is_empty() {
            return configured_path;
        }

        let dist_config = std::path::Path::new(DIST_CONFIG_PATH);

        if dist_config.exists() || std::path::Path::new("config").is_dir() {
            return dist_config.to_string_lossy().to_string();
        }

        LEGACY_CONFIG_PATH.to_string()
    }

    /// Mirrors `setupServer`.
    fn setup_server() {
        let server_ip = ServerConfiguration::get_string("server.bind");

        if server_ip.is_empty() {
            tracing::error!("Game server bind address is not provided");
            return;
        }

        let server_port = ServerConfiguration::get_integer("server.port");

        if server_port == 0 {
            tracing::error!("Game server port not provided");
            return;
        }

        *SERVER_IP.write() = server_ip;
        SERVER_PORT.store(server_port, Ordering::SeqCst);

        let server = NettyServer::get_instance();
        server.create_socket();
    }

    /// Mirrors `setupRcon`.
    fn setup_rcon() {
        let rcon_ip = ServerConfiguration::get_string("rcon.bind");

        if rcon_ip.is_empty() {
            tracing::error!("Remote control (RCON) server bind address is not provided");
            return;
        }

        let rcon_port = ServerConfiguration::get_integer("rcon.port");

        if rcon_port == 0 {
            tracing::error!("Remote control (RCON) server port not provided");
            return;
        }

        *RCON_IP.write() = rcon_ip;
        RCON_PORT.store(rcon_port, Ordering::SeqCst);

        let server = RconServer::get_instance();
        server.create_socket();
    }

    /// Mirrors `setupMus`.
    fn setup_mus() {
        let mus_server_ip = ServerConfiguration::get_string("mus.bind");

        if mus_server_ip.is_empty() {
            tracing::error!("Multi User Server (MUS) bind address is not provided");
            return;
        }

        let mus_server_port = ServerConfiguration::get_integer("mus.port");

        if mus_server_port == 0 {
            tracing::error!("Multi User Server (MUS) port not provided");
            return;
        }

        *MUS_SERVER_IP.write() = mus_server_ip;
        MUS_SERVER_PORT.store(mus_server_port, Ordering::SeqCst);

        let server = MusServer::get_instance();
        server.create_socket();
    }

    /// Mirrors `dispose`.
    pub fn dispose() {
        tracing::info!("Shutting down server!");
        Self::set_shutting_down(true);

        ChatManager::get_instance().perform_chat_saving();

        let scheduler = GameScheduler::get_instance();
        scheduler.perform_item_saving();
        scheduler.perform_item_deletion();

        PlayerManager::get_instance().dispose();

        NettyServer::get_instance().dispose();
    }

    /// Mirrors `getGson()`.
    pub fn get_gson() -> &'static Gson {
        static GSON: OnceLock<Gson> = OnceLock::new();
        GSON.get_or_init(|| Gson)
    }

    /// Mirrors `getPasswordEncoder()`.
    pub fn get_password_encoder() -> &'static PasswordEncoder {
        static ENCODER: PasswordEncoder = PasswordEncoder;
        &ENCODER
    }

    /// Mirrors `isShuttingdown()`.
    pub fn is_shutting_down() -> bool {
        SHUTTING_DOWN
            .get_or_init(|| AtomicBool::new(false))
            .load(Ordering::SeqCst)
    }

    /// Sets the shutdown flag (mirrors the Java static `isShutdown` field).
    pub fn set_shutting_down(shutting_down: bool) {
        SHUTTING_DOWN
            .get_or_init(|| AtomicBool::new(false))
            .store(shutting_down, Ordering::SeqCst);
    }

    /// Mirrors `getStartupTime()`.
    ///
    /// Java records the value in `main`; here the first call to
    /// `getStartupTime` (made at the top of `run`) plays that role.
    pub fn get_startup_time() -> i64 {
        *STARTUP_TIME.get_or_init(|| DateUtil::get_current_time_seconds() as i64)
    }

    /// Mirrors `isHappyHour()`.
    pub fn is_happy_hour() -> bool {
        let now = chrono::Local::now();
        let is_weekend =
            matches!(now.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun);
        let (start_key, end_key) = if is_weekend {
            ("happy.hour.weekend.start", "happy.hour.weekend.end")
        } else {
            ("happy.hour.weekday.start", "happy.hour.weekday.end")
        };

        let config = GameConfiguration::get_instance();
        let from = Self::parse_time_of_day(&config.get_string(start_key));
        let to = Self::parse_time_of_day(&config.get_string(end_key));
        let now_time = Self::parse_time_of_day(&DateUtil::get_current_date("HH:mm:ss"));

        match (from, to, now_time) {
            (Some(from), Some(to), Some(now_time)) => now_time > from && now_time < to,
            _ => false,
        }
    }

    fn parse_time_of_day(value: &str) -> Option<i32> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 3 {
            return None;
        }

        let hours: i32 = parts[0].parse().ok()?;
        let minutes: i32 = parts[1].parse().ok()?;
        let seconds: i32 = parts[2].parse().ok()?;
        Some(hours * 3_600 + minutes * 60 + seconds)
    }

    /// Mirrors `getServer()`.
    pub fn get_server() -> &'static NettyServer {
        NettyServer::get_instance()
    }

    /// Mirrors `getServerIP()`.
    pub fn get_server_ip() -> String {
        SERVER_IP.read().clone()
    }

    /// Mirrors `getServerPort()`.
    pub fn get_server_port() -> i32 {
        SERVER_PORT.load(Ordering::SeqCst)
    }

    /// Mirrors the Java `musServerIP` / `musServerPort` static fields.
    pub fn get_mus_server_ip() -> String {
        MUS_SERVER_IP.read().clone()
    }

    /// Mirrors the Java `musServerIP` / `musServerPort` static fields.
    pub fn get_mus_server_port() -> i32 {
        MUS_SERVER_PORT.load(Ordering::SeqCst)
    }

    /// Mirrors `getRconIP()`.
    pub fn get_rcon_ip() -> String {
        RCON_IP.read().clone()
    }

    /// Mirrors `getRconPort()`.
    pub fn get_rcon_port() -> i32 {
        RCON_PORT.load(Ordering::SeqCst)
    }
}
