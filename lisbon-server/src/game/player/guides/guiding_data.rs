//! Mirrors `net.h4bbo.lisbon.game.player.guides.GuidingData`.

#[derive(Clone, Debug)]
pub struct GuidingData {
    user_id: i32,
    username: String,
    last_online: i64,
    time_online: i64,
}

impl GuidingData {
    /// Mirrors the 4-arg `GuidingData` constructor.
    pub fn new(user_id: i32, username: &str, last_online: i64, time_online: i64) -> Self {
        Self {
            user_id,
            username: username.to_string(),
            last_online,
            time_online,
        }
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getUsername()`.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Mirrors `getLastOnline()`.
    pub fn get_last_online(&self) -> i64 {
        self.last_online
    }

    /// Mirrors `getTimeOnline()`.
    pub fn get_time_online(&self) -> i64 {
        self.time_online
    }
}
