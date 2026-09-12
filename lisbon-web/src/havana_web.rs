//! Mirrors `org.alexdev.http.HavanaWeb`.

use std::sync::OnceLock;

use lisbon_server::dao::storage::Storage;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::lisbon::Gson;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::config::server_configuration::ServerConfiguration;

use crate::duckhttpd::{Settings, WebServer};
use crate::game::news::news_manager::NewsManager;
use crate::game::stickers::sticker_manager::StickerManager;
use crate::routes::register_routes;
use crate::server::server_responses::ServerResponses;
use crate::server::watchdog::Watchdog;
use crate::template::twig_template::TwigTemplate;
use crate::util::config::web_logging_configuration::WebLoggingConfiguration;
use crate::util::config::web_server_config_writer::WebServerConfigWriter;
use crate::util::config::web_settings_config_writer::WebSettingsConfigWriter;

pub struct HavanaWeb;

const DIST_CONFIG_PATH: &str = "config/webserver-config.ini";
const LEGACY_CONFIG_PATH: &str = "webserver-config.ini";

static EXECUTOR: OnceLock<Executor> = OnceLock::new();
static WATCHDOG_THREAD: OnceLock<std::thread::JoinHandle<()>> = OnceLock::new();

/// Mirrors the Java `executor` static field
/// (`Executors.newFixedThreadPool(availableProcessors)`) — a small fixed
/// thread pool on `std::thread` + an mpsc task queue.
pub struct Executor {
    send: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

impl Executor {
    pub fn new(worker_count: usize) -> Self {
        let (send, receive) = std::sync::mpsc::channel::<Box<dyn FnOnce() + Send>>();
        let receive = std::sync::Arc::new(std::sync::Mutex::new(receive));

        for _ in 0..worker_count.max(1) {
            let receive = receive.clone();
            std::thread::spawn(move || {
                while let Ok(task) = receive.lock().unwrap().recv() {
                    task();
                }
            });
        }

        Self { send }
    }

    /// Mirrors `ExecutorService.execute(Runnable)`.
    pub fn execute(&self, task: impl FnOnce() + Send + 'static) {
        let boxed: Box<dyn FnOnce() + Send> = Box::new(task);

        if let Err(error) = self.send.send(boxed) {
            tracing::warn!("executor channel closed; running the task inline");
            error.0();
        }
    }
}

impl HavanaWeb {
    /// Mirrors `main(String[])`.
    pub fn main_entry() {
        if let Err(error) = WebLoggingConfiguration::check_logging_config() {
            tracing::warn!("logging configuration check failed: {error}");
        }

        ServerConfiguration::set_writer(Box::new(WebServerConfigWriter::new()));
        ServerConfiguration::load(&Self::resolve_web_server_config_path());

        tracing::info!("HavanaWeb by Quackster");
        tracing::info!("Loading configuration..");

        let settings = Settings::get_instance();
        settings.set_site_directory(&ServerConfiguration::get_string("site.directory"));
        settings.set_default_responses(Box::new(ServerResponses));
        settings.set_template_base(&TwigTemplate::new(None));
        settings.set_save_sessions(true);

        let page_encoding = ServerConfiguration::get_string("page.encoding");
        if !page_encoding.is_empty() {
            settings.set_page_encoding(&page_encoding);
        }

        if !Storage::connect() {
            tracing::error!("Could not connect to MySQL");
            return;
        }

        GameConfiguration::get_instance_with_writer(WebSettingsConfigWriter::new());

        WordfilterManager::get_instance();
        StickerManager::get_instance();
        ItemManager::get_instance();
        NewsManager::get_instance();

        let worker_count = std::thread::available_parallelism()
            .map(|number| number.get())
            .unwrap_or(1);
        let _ = EXECUTOR.get_or_init(|| Executor::new(worker_count));

        // Mirrors the Java `scheduler.scheduleWithFixedDelay(new Watchdog(),
        // 1, 1, TimeUnit.SECONDS)`; the fixed-delay loop runs on a plain
        // thread (the Java scheduled pool is not ported).
        let watchdog_thread = std::thread::spawn(|| {
            let mut watchdog = Watchdog::new();

            loop {
                watchdog.run();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        let _ = WATCHDOG_THREAD.set(watchdog_thread);

        tracing::info!("Registering web routes..");

        register_routes();
        tracing::info!(
            "Registered {} route(s)!",
            crate::duckhttpd::RouteManager::get_routes().len()
        );

        let port = ServerConfiguration::get_integer("bind.port");
        tracing::info!("Starting http service on port {port}");

        WebServer::new(port).start();
    }

    /// Mirrors `resolveWebServerConfigPath()`.
    fn resolve_web_server_config_path() -> String {
        let configured_path = std::env::var("LISBON_WEB_CONFIG")
            .ok()
            .filter(|value| !value.trim().is_empty());

        let configured_path = match configured_path {
            Some(value) => Some(value),
            None => std::env::var("LISBON_CONFIG")
                .ok()
                .filter(|value| !value.trim().is_empty()),
        };

        if let Some(path) = configured_path {
            return path;
        }

        if std::path::Path::new(DIST_CONFIG_PATH).exists()
            || std::path::Path::new("config").is_dir()
        {
            return DIST_CONFIG_PATH.to_string();
        }

        LEGACY_CONFIG_PATH.to_string()
    }

    /// Mirrors `getExecutor()`.
    pub fn get_executor() -> &'static Executor {
        EXECUTOR.get_or_init(|| {
            let worker_count = std::thread::available_parallelism()
                .map(|number| number.get())
                .unwrap_or(1);
            Executor::new(worker_count)
        })
    }

    /// Mirrors `getGson()`.
    pub fn get_gson() -> &'static Gson {
        lisbon_server::lisbon::Lisbon::get_gson()
    }

    /// Mirrors `hashSpriteName(String)`.
    pub fn hash_sprite_name(name: &str) -> i64 {
        let mut hash: i64 = 0;

        for character in name.to_uppercase().chars() {
            hash = hash.wrapping_mul(61) + character as i64 - 32;
            hash = hash.wrapping_add(((hash as u64) >> 56) as i64);
        }

        hash
    }
}
