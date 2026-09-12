//! Mirrors `net.h4bbo.lisbon.game.texts.TextsManager`.
use std::collections::HashMap;
use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::dao::mysql::settings_dao::SettingsDao;

pub struct TextsManager;

impl TextsManager {
    /// Mirrors the Java constructor's `SettingsDao.getTexts()` load.
    fn texts() -> &'static Mutex<HashMap<String, String>> {
        static TEXTS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
        TEXTS.get_or_init(|| Mutex::new(SettingsDao::get_texts()))
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static TextsManager {
        static INSTANCE: OnceLock<TextsManager> = OnceLock::new();
        INSTANCE.get_or_init(|| TextsManager)
    }

    /// Mirrors `getValue(String)`.
    pub fn get_value(&self, key: &str) -> String {
        Self::texts()
            .lock()
            .get(key)
            .cloned()
            .unwrap_or_default()
    }

    /// Mirrors `reset()` (reloads the texts map).
    pub fn reset() {
        *Self::texts().lock() = SettingsDao::get_texts();
    }
}
