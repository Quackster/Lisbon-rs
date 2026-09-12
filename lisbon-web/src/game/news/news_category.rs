//! Mirrors `org.alexdev.http.game.news.NewsCategory`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct NewsCategory {
    pub id: i32,
    pub label: String,
    pub index: String,
}

impl NewsCategory {
    /// Mirrors the `NewsCategory(int, String, String)` constructor.
    pub fn new(id: i32, label: &str, index: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
            index: index.to_string(),
        }
    }
}
