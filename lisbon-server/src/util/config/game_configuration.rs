//! Mirrors `net.h4bbo.lisbon.util.config.GameConfiguration`.
//!
//! Java is a mutable singleton. Here we keep a global `Arc`-backed instance.

use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::{Mutex, MutexGuard, RwLock};

use serde::{Serialize, Serializer};

use super::writer::{ConfigWriter, GameConfigWriter};

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<GameConfiguration>>> = RwLock::new(None);
}

pub struct GameConfiguration {
    config: Mutex<HashMap<String, String>>,
}

impl GameConfiguration {
    /// Mirrors the constructor.
    pub fn new(config_writer: &dyn ConfigWriter) -> Self {
        let mut config = config_writer.set_configuration_defaults();

        let settings = crate::dao::mysql::settings_dao::SettingsDao::get_all_settings();
        for (key, value) in config.clone().iter() {
            match settings.get(key) {
                Some(db_value) => {
                    config.insert(key.clone(), db_value.clone());
                }
                None => {
                    crate::dao::mysql::settings_dao::SettingsDao::new_setting(key, value);
                }
            }
        }

        Self {
            config: Mutex::new(config),
        }
    }

    /// Returns the current instance, creating it (with a default
    /// `GameConfigWriter`) if it has not been initialised.
    pub fn get_instance() -> Arc<GameConfiguration> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let writer = GameConfigWriter::new();
        let instance = Arc::new(GameConfiguration::new(&writer));
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `getInstance(ConfigWriter)` — (re)creates the instance with the
    /// given writer.
    pub fn get_instance_with_writer(
        writer: impl ConfigWriter + 'static,
    ) -> Arc<GameConfiguration> {
        let writer = Box::new(writer);
        let instance = Arc::new(GameConfiguration::new(writer.as_ref()));
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset(ConfigWriter)`.
    pub fn reset(writer: impl ConfigWriter + 'static) -> Arc<GameConfiguration> {
        Self::get_instance_with_writer(writer)
    }

    /// Mirrors `getConfig`.
    pub fn get_config(&self) -> MutexGuard<'_, HashMap<String, String>> {
        self.config.lock()
    }

    /// Mirrors `getBoolean`.
    pub fn get_bool(&self, key: &str) -> bool {
        let config = self.config.lock();
        let val = config.get(key).cloned().unwrap_or_else(|| "false".to_string());
        if val.eq_ignore_ascii_case("true") {
            return true;
        }
        if val == "1" {
            return true;
        }
        val.eq_ignore_ascii_case("yes")
    }

    /// Mirrors `getString(key)`.
    pub fn get_string(&self, key: &str) -> String {
        self.config
            .lock()
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Mirrors `getString(key, def)`.
    pub fn get_string_default(&self, key: &str, def: &str) -> String {
        self.config
            .lock()
            .get(key)
            .cloned()
            .unwrap_or_else(|| def.to_string())
    }

    /// Mirrors `getInteger`.
    pub fn get_integer(&self, key: &str) -> i32 {
        self.config
            .lock()
            .get(key)
            .cloned()
            .unwrap_or_else(|| "0".to_string())
            .parse::<i32>()
            .unwrap_or(0)
    }

    /// Mirrors `getLong`.
    pub fn get_long(&self, key: &str) -> i64 {
        self.config
            .lock()
            .get(key)
            .cloned()
            .unwrap_or_else(|| "0".to_string())
            .parse::<i64>()
            .unwrap_or(0)
    }

    /// Mirrors `exists`.
    pub fn exists(&self, key: &str) -> bool {
        self.config.lock().contains_key(key)
    }
}

impl Serialize for GameConfiguration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.config.lock().serialize(serializer)
    }
}
