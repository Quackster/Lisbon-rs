//! Mirrors `net.h4bbo.lisbon.game.item.Transaction`.
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Transaction {
    description: String,
    cost_coins: i32,
    cost_pixels: i32,
    amount: i32,
    created_at: i64,
    item_id: i32,
}

impl Transaction {
    /// Mirrors the `Transaction(String[], String, int, int, int, long)`
    /// constructor.
    pub fn new(
        item_ids: &[String],
        description: &str,
        cost_coins: i32,
        cost_pixels: i32,
        amount: i32,
        created_at: i64,
    ) -> Self {
        let item_id = if let Some(first) = item_ids.first() {
            if !first.is_empty() && first.chars().all(|c| c.is_ascii_digit()) {
                first.parse().unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };

        Self {
            description: description.to_string(),
            cost_coins,
            cost_pixels,
            amount,
            created_at,
            item_id,
        }
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> i32 {
        self.item_id
    }

    /// Mirrors `getFormattedDate()`.
    pub fn get_formatted_date(&self) -> String {
        DateUtil::get_date(self.created_at, "yyyy-MM-dd HH:mm a")
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getDescription()`.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Mirrors `getCostCoins()`.
    pub fn get_cost_coins(&self) -> i32 {
        self.cost_coins
    }

    /// Mirrors `getCostPixels()`.
    pub fn get_cost_pixels(&self) -> i32 {
        self.cost_pixels
    }

    /// Mirrors `getAmount()`.
    pub fn get_amount(&self) -> i32 {
        self.amount
    }
}
