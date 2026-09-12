//! Mirrors `org.alexdev.http.game.collectables.CollectableEntry`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct CollectableEntry {
    pub sprite: String,
    pub name: String,
    pub description: String,
}

impl CollectableEntry {
    /// Mirrors the `CollectableEntry(String, String, String)` constructor.
    pub fn new(sprite: &str, name: &str, description: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
