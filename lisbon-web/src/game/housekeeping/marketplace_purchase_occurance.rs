//! Mirrors `org.alexdev.http.game.housekeeping.MarketplacePurchaseOccurance`.

#[derive(Clone, Debug)]
pub struct MarketplacePurchaseOccurance {
    pub user_id: i32,
    pub username: String,
    pub purchase_count: i32,
}

impl MarketplacePurchaseOccurance {
    /// Mirrors the `MarketplacePurchaseOccurance(int, String, int)` constructor.
    pub fn new(user_id: i32, username: &str, purchase_count: i32) -> Self {
        Self {
            user_id,
            username: username.to_string(),
            purchase_count,
        }
    }
}
