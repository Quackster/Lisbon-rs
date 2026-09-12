//! Mirrors `net.h4bbo.lisbon.game.achievements.AchievementInfo`.

#[derive(Clone, Debug)]
pub struct AchievementInfo {
    id: i32,
    name: String,
    level: i32,
    pixel_reward: i32,
    progress_required: i32,
}

impl AchievementInfo {
    /// Mirrors the `AchievementInfo(int, String, int, int, int)` constructor.
    pub fn new(
        id: i32,
        name: &str,
        level: i32,
        pixel_reward: i32,
        progress_required: i32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            level,
            pixel_reward,
            progress_required,
        }
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getLevel`.
    pub fn get_level(&self) -> i32 {
        self.level
    }

    /// Mirrors `getPixelReward`.
    pub fn get_pixel_reward(&self) -> i32 {
        self.pixel_reward
    }

    /// Mirrors `getProgressRequired`.
    pub fn get_progress_required(&self) -> i32 {
        self.progress_required
    }
}
