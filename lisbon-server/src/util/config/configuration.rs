//! Mirrors `net.h4bbo.lisbon.util.config.Configuration`.
//!
//! Java uses `commons-configuration2`'s `INIConfiguration`. Here we use a small
//! self-contained INI parser. Keys are flattened to `key -> value` (section
//! names are not part of the key), mirroring the Java behaviour: section
//! headers are ignored and `".."` in keys is collapsed to `"."`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Configuration;

impl Configuration {
    /// Mirrors `Configuration.load` — parses an INI file and returns the
    /// flattened `key -> value` map.
    pub fn load(config_path: &str) -> HashMap<String, String> {
        let mut config: HashMap<String, String> = HashMap::new();
        let contents = fs::read_to_string(config_path).unwrap_or_default();

        for line in contents.lines() {
            let line = line.trim();

            if line.is_empty()
                || line.starts_with('#')
                || line.starts_with(';')
                || line.starts_with('[')
            {
                continue;
            }

            if let Some(eq) = line.find('=') {
                let key = line[..eq].trim().replace("..", ".");
                let value = line[eq + 1..].trim();
                if !key.is_empty() {
                    config.insert(key, value.to_string());
                }
            }
        }

        config
    }

    /// Mirrors `Configuration.createConfigurationFile` — creates the file if it
    /// does not already exist and returns its path. Returns `None` when the
    /// file already existed (Java returns `null` in that case).
    pub fn create_configuration_file(config_path: &str) -> Option<PathBuf> {
        let path = std::path::Path::new(config_path);

        if path.is_file() {
            return None;
        }

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::File::create(path) {
            Ok(_) => Some(path.to_path_buf()),
            Err(_) => None,
        }
    }
}
