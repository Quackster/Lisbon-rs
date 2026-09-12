//! Mirrors `org.alexdev.http.util.ConfigEntry`.

/// Mirrors `org.alexdev.http.util.ConfigEntry`.
pub struct ConfigEntry {
    key: String,
    value: String,
}

impl ConfigEntry {
    /// Mirrors `ConfigEntry(String, String)`.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Mirrors `getKey()`.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Mirrors `setKey(String)`.
    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = key.into();
    }

    /// Mirrors `getValue()`.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Mirrors `setValue(String)`.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}
