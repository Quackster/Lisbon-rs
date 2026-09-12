//! Mirrors `net.h4bbo.lisbon.game.badges.Badge`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct Badge {
    badge_code: String,
    equipped: bool,
    slot_id: i32,
}

impl Badge {
    /// Mirrors the `Badge(String, boolean, int)` constructor.
    pub fn new(badge_code: &str, equipped: bool, slot_id: i32) -> Self {
        Self {
            badge_code: badge_code.to_string(),
            equipped,
            slot_id,
        }
    }

    /// Mirrors `getBadgeCode()`.
    pub fn get_badge_code(&self) -> &str {
        &self.badge_code
    }

    /// Mirrors `isEquipped()`.
    pub fn is_equipped(&self) -> bool {
        self.equipped
    }

    /// Mirrors `setEquipped(boolean)`.
    pub fn set_equipped(&mut self, equipped: bool) {
        self.equipped = equipped;
    }

    /// Mirrors `getSlotId()`.
    pub fn get_slot_id(&self) -> i32 {
        self.slot_id
    }

    /// Mirrors `setSlotId(int)`.
    pub fn set_slot_id(&mut self, slot_id: i32) {
        self.slot_id = slot_id;
    }
}
