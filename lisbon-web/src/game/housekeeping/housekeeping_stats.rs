//! Mirrors `org.alexdev.http.game.housekeeping.HousekeepingStats`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct HousekeepingStats {
    pub user_count: i32,
    pub inventory_items_count: i32,
    pub room_item_count: i32,
    pub group_count: i32,
    pub pet_count: i32,
    pub photo_count: i32,
}

impl HousekeepingStats {
    /// Mirrors the `HousekeepingStats(int, int, int, int, int, int)` constructor.
    pub fn new(
        user_count: i32,
        inventory_items_count: i32,
        room_item_count: i32,
        group_count: i32,
        pet_count: i32,
        photo_count: i32,
    ) -> Self {
        Self {
            user_count,
            inventory_items_count,
            room_item_count,
            group_count,
            pet_count,
            photo_count,
        }
    }
}
