//! Mirrors `org.alexdev.http.game.homes.Home`.

use crate::dao::homes_dao::HomesDao;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Home {
    pub user_id: i32,
    pub background: String,
}

impl Home {
    /// Mirrors the `Home(int, String)` constructor.
    pub fn new(user_id: i32, background: &str) -> Self {
        Self {
            user_id,
            background: background.to_string(),
        }
    }

    /// Mirrors `setBackground(String)`.
    pub fn set_background(&mut self, background: &str) {
        self.background = background.to_string();
    }

    /// Mirrors `saveBackground()`.
    pub fn save_background(&self) {
        HomesDao::save_background(self.user_id, &self.background);
    }
}
