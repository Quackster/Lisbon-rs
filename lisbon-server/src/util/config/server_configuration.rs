//! Mirrors `net.h4bbo.lisbon.util.config.ServerConfiguration`.
//!
//! Java uses static fields; here we back the global state with `lazy_static`
//! `RwLock`s.

use std::collections::HashMap;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use super::configuration::Configuration;
use super::writer::ConfigWriter;

lazy_static! {
    static ref CONFIG: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
    static ref WRITER: RwLock<Option<Box<dyn ConfigWriter + Send + Sync>>> = RwLock::new(None);
}

pub struct ServerConfiguration;

impl ServerConfiguration {
    /// Mirrors `setWriter`.
    pub fn set_writer(writer: Box<dyn ConfigWriter + Send + Sync>) {
        *WRITER.write() = Some(writer);
    }

    /// Mirrors `load`.
    pub fn load(config_path: &str) {
        let defaults = {
            let writer = WRITER.read();
            match writer.as_ref() {
                Some(w) => w.set_configuration_defaults(),
                None => HashMap::new(),
            }
        };

        if let Some(path) = Configuration::create_configuration_file(config_path) {
            if let Ok(mut file) = std::fs::File::create(&path) {
                let writer = WRITER.read();
                if let Some(w) = writer.as_ref() {
                    let _ = w.set_configuration_data(&defaults, &mut file);
                }
            }
        }

        *CONFIG.write() = Configuration::load(config_path);
    }

    /// Mirrors `getEnvOrNull`.
    fn get_env_or_null(key: &str) -> Option<String> {
        let env_key = key.replace('.', "_").to_uppercase();
        match std::env::var(&env_key) {
            Ok(v) if !v.is_empty() => Some(v),
            _ => None,
        }
    }

    /// Mirrors `getString`.
    pub fn get_string(key: &str) -> String {
        if let Some(v) = Self::get_env_or_null(key) {
            return v;
        }
        CONFIG.read().get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    /// Mirrors `getStringOrDefault`.
    pub fn get_string_or_default(key: &str, value: &str) -> String {
        if let Some(v) = Self::get_env_or_null(key) {
            return v;
        }
        CONFIG.read().get(key).cloned().unwrap_or_else(|| value.to_string())
    }

    /// Mirrors `getInteger`.
    pub fn get_integer(key: &str) -> i32 {
        if let Some(v) = Self::get_env_or_null(key) {
            if let Ok(p) = v.parse::<i32>() {
                return p;
            }
            eprintln!("Environment variable for key {} is not a valid integer: {}", key, v);
        }
        CONFIG
            .read()
            .get(key)
            .cloned()
            .unwrap_or_else(|| "0".to_string())
            .parse::<i32>()
            .unwrap_or(0)
    }

    /// Mirrors `getBoolean`.
    pub fn get_boolean(key: &str) -> bool {
        let val = CONFIG
            .read()
            .get(key)
            .cloned()
            .unwrap_or_else(|| "false".to_string());

        if val.eq_ignore_ascii_case("true") {
            return true;
        }
        if val == "1" {
            return true;
        }
        val.eq_ignore_ascii_case("yes")
    }

    /// Mirrors `exists`.
    pub fn exists(key: &str) -> bool {
        CONFIG.read().contains_key(key)
    }
}
