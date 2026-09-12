//! Mirrors `org.alexdev.http.server.Watchdog`.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use lazy_static::lazy_static;
use parking_lot::RwLock;

use lisbon_server::dao::mysql::events_dao::EventsDao;
use lisbon_server::dao::mysql::settings_dao::SettingsDao;
use lisbon_server::game::catalogue::catalogue_manager::CatalogueManager;
use lisbon_server::game::events::event::Event;
use lisbon_server::game::groups::group::Group;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::game::room::room::Room;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::config::server_configuration::ServerConfiguration;

use crate::dao::email_dao::EmailDao;
use crate::util::config::web_settings_config_writer::WebSettingsConfigWriter;

lazy_static! {
    pub static ref STAFF_PICK_GROUPS: RwLock<Vec<Group>> = RwLock::new(Vec::new());
    pub static ref RECOMMENDED_GROUPS: RwLock<Vec<Group>> = RwLock::new(Vec::new());
    pub static ref EVENTS: RwLock<Vec<Event>> = RwLock::new(Vec::new());
    pub static ref RECOMMENDED_ROOMS: RwLock<Vec<Room>> = RwLock::new(Vec::new());
    pub static ref HIDDEN_RECOMMENDED_ROOMS: RwLock<Vec<Room>> = RwLock::new(Vec::new());
    pub static ref RECENT_DISCUSSIONS: RwLock<Vec<crate::game::groups::discussion_topic::DiscussionTopic>> = RwLock::new(Vec::new());
    pub static ref NEXT_RECENT_DISCUSSIONS: RwLock<Vec<crate::game::groups::discussion_topic::DiscussionTopic>> = RwLock::new(Vec::new());
    pub static ref TAG_CLOUD_10: RwLock<Vec<(String, i32)>> = RwLock::new(Vec::new());
    pub static ref TAG_CLOUD_20: RwLock<Vec<(String, i32)>> = RwLock::new(Vec::new());
    pub static ref NEWS: RwLock<Vec<crate::game::news::news_article::NewsArticle>> = RwLock::new(Vec::new());
    pub static ref NEWS_STAFF: RwLock<Vec<crate::game::news::news_article::NewsArticle>> = RwLock::new(Vec::new());
    pub static ref USERS_ONLNE: AtomicI32 = AtomicI32::new(0);
    pub static ref IS_SERVER_ONLINE: AtomicBool = AtomicBool::new(false);
    pub static ref LAST_VISITS: AtomicI32 = AtomicI32::new(0);
}

/// Mirrors `org.alexdev.http.server.Watchdog`.
pub struct Watchdog {
    can_reset_users_flag: bool,
    has_reset_users_flag: bool,
    counter: AtomicI32,
}

impl Watchdog {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            can_reset_users_flag: true,
            has_reset_users_flag: false,
            counter: AtomicI32::new(0),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        if self.counter.load(Ordering::SeqCst) % 300 == 0 {
            crate::dao::news_dao::NewsDao::publish_future_articles();
        }

        if self.counter.load(Ordering::SeqCst) % 3600 == 0 {
            EmailDao::remove_recovery_code_batch();
        }

        if self.counter.load(Ordering::SeqCst) % 30 == 0 {
            let host = ServerConfiguration::get_string("rcon.ip");
            let port = ServerConfiguration::get_integer("rcon.port");
            IS_SERVER_ONLINE.store(self.is_server_online(&host, port), Ordering::SeqCst);
            USERS_ONLNE.store(
                SettingsDao::get_setting("players.online")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0),
                Ordering::SeqCst,
            );
            LAST_VISITS.store(crate::dao::site_dao::SiteDao::get_last_visits(), Ordering::SeqCst);

            if !IS_SERVER_ONLINE.load(Ordering::SeqCst) {
                USERS_ONLNE.store(0, Ordering::SeqCst);

                if self.can_reset_users_flag {
                    self.can_reset_users_flag = false;
                    self.has_reset_users_flag = true;
                }
            } else {
                self.can_reset_users_flag = true;
            }

            if self.has_reset_users_flag {
                self.has_reset_users_flag = false;
                lisbon_server::dao::mysql::player_dao::PlayerDao::reset_online();
            }

            *EVENTS.write() = EventsDao::get_events();
            *RECOMMENDED_GROUPS.write() = crate::dao::recommended_dao::RecommendedDao::get_recommended_groups(false);
            *STAFF_PICK_GROUPS.write() = crate::dao::recommended_dao::RecommendedDao::get_recommended_groups(true);
            *RECOMMENDED_ROOMS.write() = lisbon_server::dao::mysql::room_dao::RoomDao::get_recommended_rooms(5, 0);
            *HIDDEN_RECOMMENDED_ROOMS.write() = lisbon_server::dao::mysql::room_dao::RoomDao::get_recommended_rooms(5, 5);
            *RECENT_DISCUSSIONS.write() = crate::dao::community_dao::CommunityDao::get_recent_discussions(10, 0);
            *NEXT_RECENT_DISCUSSIONS.write() = crate::dao::community_dao::CommunityDao::get_recent_discussions(10, 10);
            *TAG_CLOUD_10.write() = lisbon_server::dao::mysql::tag_dao::TagDao::get_popular_tags(10);
            *TAG_CLOUD_20.write() = lisbon_server::dao::mysql::tag_dao::TagDao::get_popular_tags(20);
            *NEWS.write() = crate::dao::news_dao::NewsDao::get_top(crate::game::news::news_date_key::NewsDateKey::All, 5, false, &[], 0);
            *NEWS_STAFF.write() = crate::dao::news_dao::NewsDao::get_top(crate::game::news::news_date_key::NewsDateKey::All, 5, true, &[], 0);
        }

        if self.counter.load(Ordering::SeqCst) % 3600 == 0 {
            ItemManager::reset();
            CatalogueManager::reset();
        }

        // reload the config every 30 seconds
        if self.counter.load(Ordering::SeqCst) % 30 == 0 {
            GameConfiguration::get_instance_with_writer(WebSettingsConfigWriter::new());
        }

        self.reset_counter();
    }

    /// Mirrors `resetCounter()`.
    fn reset_counter(&mut self) {
        let _ = self.counter.fetch_add(1, Ordering::SeqCst);
    }

    /// Mirrors `isServerOnline(String, int)`.
    pub fn is_server_online(&self, host: &str, port: i32) -> bool {
        if GameConfiguration::get_instance().get_bool("hotel.check.online") {
            self.is_host_online(host, port)
        } else {
            true
        }
    }

    /// Mirrors `isHostOnline(String, int)`.
    pub fn is_host_online(&self, host: &str, port: i32) -> bool {
        std::net::TcpStream::connect((host, port as u16)).is_ok()
    }
}
