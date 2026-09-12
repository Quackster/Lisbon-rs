//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerCategory`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct MessengerCategory {
    id: i32,
    user_id: i32,
    name: String,
}

impl MessengerCategory {
    /// Mirrors the `MessengerCategory(int, int, String)` constructor.
    pub fn new(id: i32, user_id: i32, name: &str) -> Self {
        Self {
            id,
            user_id,
            name: name.to_string(),
        }
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getUserId`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
