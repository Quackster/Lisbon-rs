//! Mirrors `net.h4bbo.lisbon.dao.mysql.SettingsDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct SettingsDao;

impl SettingsDao {
    /// Mirrors `updateSettings(Set<Map.Entry<String, String>>)`.
    pub fn update_settings(entries: &[(String, String)]) {
        for (key, value) in entries {
            Storage::get_storage().execute(&format!(
                "UPDATE settings SET value = '{}' WHERE setting = '{}'",
                escape(value),
                escape(key)
            ));
        }
    }

    /// Mirrors `updateSetting(String, String)`.
    pub fn update_setting(key: &str, value: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE settings SET value = '{}' WHERE setting = '{}'",
            escape(value),
            escape(key)
        ));
    }

    /// Mirrors `getSetting(String)`.
    pub fn get_setting(key: &str) -> Option<String> {
        Storage::get_storage().get_string(
            &format!("SELECT value FROM settings WHERE setting = '{}'", escape(key)),
            "value",
        )
    }

    /// Mirrors `getAllSettings()`.
    pub fn get_all_settings() -> HashMap<String, String> {
        let mut settings = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT setting, value FROM settings") {
            if let (Some(setting), Some(value)) = (row.str("setting"), row.str("value")) {
                settings.insert(setting, value);
            }
        }

        settings
    }

    /// Mirrors `newSetting(String, String)`.
    pub fn new_setting(key: &str, value: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO settings (setting, value) VALUES ('{}', '{}')",
            escape(key),
            escape(value)
        ));
    }

    /// Mirrors `getTexts()`.
    pub fn get_texts() -> HashMap<String, String> {
        let mut texts = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM external_texts") {
            if let (Some(entry), Some(text)) = (row.str("entry"), row.str("text")) {
                texts.insert(entry, text);
            }
        }

        texts
    }
}
