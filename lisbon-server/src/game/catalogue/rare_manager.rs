//! Mirrors `net.h4bbo.lisbon.game.catalogue.RareManager`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::OnceLock;

use parking_lot::Mutex;
use rand::seq::SliceRandom;

use crate::dao::mysql::rare_dao::RareDao;
use crate::dao::mysql::settings_dao::SettingsDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

const RARE_TICK_SETTING: &str = "rare.cycle.tick.time";

pub struct RareManager {
    rare_list: Mutex<Vec<CatalogueItem>>,
    rare_cost: Mutex<HashMap<i32, i32>>,
    days_since_used: Mutex<HashMap<String, i64>>,
    current_rare: Mutex<Option<CatalogueItem>>,
    tick_time: AtomicI64,
}

impl RareManager {
    fn new() -> Self {
        let tick_time = AtomicI64::new(GameConfiguration::get_instance().get_long(RARE_TICK_SETTING));
        let mut rare_cost = HashMap::new();

        for numbers in GameConfiguration::get_instance()
            .get_string("rare.cycle.pages")
            .split('|')
        {
            let parts: Vec<&str> = numbers.split(',').collect();

            if parts.len() >= 2 {
                let catalogue_page = parts[0].parse::<i32>().unwrap_or(0);
                let hours_required = parts[1].parse::<i32>().unwrap_or(0);

                if hours_required > 0 {
                    for item in CatalogueManager::get_instance()
                        .get_catalogue_page_items(catalogue_page, true)
                    {
                        rare_cost.insert(
                            item.get_id(),
                            Self::get_handout_amount_in_hours(hours_required),
                        );
                    }
                }
            }
        }

        let days_since_used = RareDao::get_used_rares();
        let mut current_rare = None;

        if !days_since_used.is_empty() {
            if let Some((sale_code, _)) = RareDao::get_current_rare() {
                current_rare = CatalogueManager::get_instance().get_catalogue_item(&sale_code);
            }
        }

        let manager = Self {
            rare_list: Mutex::new(Vec::new()),
            rare_cost: Mutex::new(rare_cost),
            days_since_used: Mutex::new(days_since_used),
            current_rare: Mutex::new(current_rare),
            tick_time,
        };
        manager.load_rares();

        if manager.current_rare.lock().is_none() {
            manager.select_new_rare();
        }

        manager
    }

    /// Mirrors `loadRares()`.
    fn load_rares(&self) {
        let mut rare_list = Vec::new();

        for catalogue_page in CatalogueManager::get_instance().get_catalogue_pages() {
            if !(catalogue_page.get_min_role().rank_id() > 1) {
                continue;
            }

            if catalogue_page.get_layout() != "ctlg_layout2"
                || catalogue_page.get_image_headline() != "catalog_rares_headline1"
            {
                continue;
            }

            rare_list.extend(
                CatalogueManager::get_instance().get_catalogue_page_items(catalogue_page.get_id(), true),
            );
        }

        rare_list.shuffle(&mut rand::thread_rng());
        self.rare_list.lock().clear();
        self.rare_list.lock().extend(rare_list);
    }

    /// Mirrors `selectNewRare()`.
    pub fn select_new_rare(&self) {
        let config = GameConfiguration::get_instance();
        let reuse_time_unit = config.get_string("rare.cycle.reuse.timeunit");
        let interval = Self::time_unit_to_seconds(
            &reuse_time_unit,
            config.get_integer("rare.cycle.reuse.interval"),
        );

        let mut to_remove = Vec::new();

        for (sprite, expiry) in self.days_since_used.lock().iter() {
            if DateUtil::get_current_time_seconds() as i64 > *expiry {
                to_remove.push(sprite.clone());
            }
        }

        for sprite in &to_remove {
            self.days_since_used.lock().remove(sprite);
        }

        RareDao::remove_rares(to_remove);

        if self.rare_list.lock().is_empty() {
            self.load_rares();
        }

        let rare = self.rare_list.lock().pop();

        if let Some(rare) = rare {
            let sprite = rare
                .get_definition()
                .map(|definition| definition.get_sprite().to_string())
                .unwrap_or_default();

            if self.days_since_used.lock().contains_key(&sprite) {
                *self.current_rare.lock() = None;

                if self.rare_list.lock().len() > 0 {
                    self.select_new_rare();
                }

                return;
            }

            *self.current_rare.lock() = Some(rare.clone());

            let override_unit_key = format!("rare.cycle.reuse.{}.timeunit", rare.get_sale_code());
            let interval = if config.exists(&override_unit_key) {
                let unit = config.get_string(&override_unit_key);
                Self::time_unit_to_seconds(
                    &unit,
                    config.get_integer(&format!("rare.cycle.reuse.{}.interval", rare.get_sale_code())),
                )
            } else {
                interval
            };

            let expiry = DateUtil::get_current_time_seconds() as i64 + interval;

            self.days_since_used.lock().insert(sprite.clone(), expiry);

            RareDao::remove_rares(vec![sprite.clone()]);
            RareDao::add_rare(&sprite, expiry);

            self.tick_time.store(0, Ordering::SeqCst);
            self.save_tick();
        }
    }

    /// Mirrors `getHandoutAmountInHours(int)`.
    pub fn get_handout_amount_in_hours(hours: i32) -> i32 {
        let config = GameConfiguration::get_instance();
        let unit = config.get_string("credits.scheduler.timeunit");
        let interval = Self::time_unit_to_minutes(
            &unit,
            config.get_integer("credits.scheduler.interval"),
        );

        let minutes_in_hour: i64 = 60;
        let minutes = minutes_in_hour / interval;

        ((hours as i64 * minutes) * config.get_integer("credits.scheduler.amount") as i64) as i32
    }

    /// Mirrors `performRareManagerJob(AtomicLong)`.
    pub fn perform_rare_manager_job(&self, tick_time: &AtomicI64) {
        let config = GameConfiguration::get_instance();
        let unit = config.get_string("rare.cycle.refresh.timeunit");
        let interval = Self::time_unit_to_seconds(
            &unit,
            config.get_integer("rare.cycle.refresh.interval"),
        );

        self.tick_time.fetch_add(1, Ordering::SeqCst);

        if tick_time.load(Ordering::SeqCst) % 60 == 0 {
            self.save_tick();
        }

        if self.tick_time.load(Ordering::SeqCst) >= interval {
            self.select_new_rare();
        }
    }

    /// Mirrors `stripColor(String)`.
    #[allow(dead_code)]
    fn strip_color(sprite: &str) -> String {
        if sprite.contains('*') {
            sprite.split('*').next().unwrap_or(sprite).to_string()
        } else {
            sprite.to_string()
        }
    }

    /// Mirrors `getCurrentRare()`.
    pub fn get_current_rare(&self) -> Option<CatalogueItem> {
        self.current_rare.lock().clone()
    }

    /// Mirrors `getRareCost()`.
    pub fn get_rare_cost(&self) -> HashMap<i32, i32> {
        self.rare_cost.lock().clone()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static RareManager {
        static INSTANCE: OnceLock<RareManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Mirrors `getTick()`.
    pub fn get_tick(&self) -> &AtomicI64 {
        &self.tick_time
    }

    /// Mirrors `saveTick()`.
    pub fn save_tick(&self) {
        let config = GameConfiguration::get_instance();
        config
            .get_config()
            .insert(RARE_TICK_SETTING.to_string(), self.tick_time.load(Ordering::SeqCst).to_string());
        SettingsDao::update_setting(
            RARE_TICK_SETTING,
            &config.get_string(RARE_TICK_SETTING),
        );
    }

    fn time_unit_to_seconds(unit: &str, value: i32) -> i64 {
        let value = value as i64;
        match unit.to_uppercase().as_str() {
            "NANOSECONDS" => value / 1_000_000_000,
            "MICROSECONDS" => value / 1_000_000,
            "MILLISECONDS" => value / 1_000,
            "SECONDS" => value,
            "MINUTES" => value * 60,
            "HOURS" => value * 3600,
            "DAYS" => value * 86400,
            _ => 0,
        }
    }

    fn time_unit_to_minutes(unit: &str, value: i32) -> i64 {
        let value = value as i64;
        match unit.to_uppercase().as_str() {
            "NANOSECONDS" => value / 60_000_000,
            "MICROSECONDS" => value / 60_000,
            "MILLISECONDS" => value / 60,
            "SECONDS" => value / 60,
            "MINUTES" => value,
            "HOURS" => value * 60,
            "DAYS" => value * 1440,
            _ => 0,
        }
    }
}
