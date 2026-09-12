//! Mirrors `net.h4bbo.lisbon.game.badges.BadgeManager`.

use crate::dao::mysql::badge_dao::BadgeDao;
use crate::game::badges::badge::Badge;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::badges::available_badges::AVAILABLE_BADGES;

#[derive(Default)]
pub struct BadgeManager {
    user_id: i32,
}

impl BadgeManager {
    /// Mirrors the `BadgeManager(int)` constructor.
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }

    /// Mirrors `loadBadges(Player)` (the Rust `BadgeManager` holds no
    /// `Player` back-reference; the Java cached badge list is served live
    /// by `getBadges`).
    pub fn load_badges(&mut self, user_id: i32) {
        self.user_id = user_id;
    }

    /// Mirrors `getBadges()`.
    pub fn get_badges(&self) -> Vec<Badge> {
        BadgeDao::get_badges(self.user_id)
    }

    /// Mirrors `getEquippedBadges()`.
    pub fn get_equipped_badges(&self) -> Vec<Badge> {
        self.get_badges()
            .into_iter()
            .filter(|badge| badge.is_equipped())
            .collect()
    }

    /// Mirrors `hasBadge(String)`.
    pub fn has_badge(&self, badge_code: &str) -> bool {
        self.get_badges()
            .iter()
            .any(|badge| badge.get_badge_code().eq_ignore_ascii_case(badge_code))
    }

    /// Mirrors `tryAddBadge(String, String, int)` (the Java
    /// `ACHIEVEMENT_NOTIFICATION` send needs the `Player` back-reference
    /// which the Rust `BadgeManager` does not hold).
    pub fn try_add_badge(&self, badge_code: &str, badge_remove: Option<&str>, _level: i32) -> Option<Badge> {
        if self.has_badge(badge_code) {
            return None;
        }

        if let Some(badge_remove) = badge_remove {
            self.remove_badge(badge_remove);
        }

        let badge = Badge::new(badge_code, false, 0);

        BadgeDao::new_badge(self.user_id, badge_code);

        Some(badge)
    }

    /// Mirrors `refreshBadges()`.
    pub fn refresh_badges(&self, player: &Player) {
        player.send(&AVAILABLE_BADGES::new(
            self.get_badges(),
            self.get_equipped_badges(),
        ));
    }

    /// Mirrors `changeBadge(String, boolean, int)` (the Java queued save is
    /// folded into an immediate `BadgeDao` write, since the Rust manager
    /// holds no badge cache).
    pub fn change_badge(&self, badge_code: &str, equipped: bool, slot_id: i32) {
        if !self.has_badge(badge_code) {
            return;
        }

        BadgeDao::save_badge_changes(self.user_id, badge_code, equipped, slot_id);
    }

    /// Mirrors `saveQueuedBadges()` (the Rust manager writes immediately, so
    /// there is no queue to flush).
    pub fn save_queued_badges(&self) {}

    /// Mirrors `removeBadge(String)`.
    pub fn remove_badge(&self, badge_code: &str) -> Option<Badge> {
        if !self.has_badge(badge_code) {
            return None;
        }

        let badge = self
            .get_badges()
            .into_iter()
            .find(|badge| badge.get_badge_code().eq_ignore_ascii_case(badge_code))?;

        BadgeDao::remove_badge(self.user_id, badge_code);

        Some(badge)
    }
}
