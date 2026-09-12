//! Mirrors `org.alexdev.http.game.marketplace.MarketplaceOffer`.

use lisbon_server::game::item::base::item_definition::ItemDefinition;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::util::date_util::DateUtil;

#[derive(Clone, Debug)]
pub struct MarketplaceOffer {
    pub id: i64,
    pub user_id: i32,
    pub item_id: i64,
    pub definition_id: i32,
    pub price: i32,
    pub lowest_price: i32,
    pub rotation: i32,
    pub state: i32,
    pub colour: i32,
    pub created_at: i64,
    pub is_active: bool,
    pub items_alike: i32,
}

impl MarketplaceOffer {
    /// Mirrors the `MarketplaceOffer(long, int, long, int, int, int, int, int, int, int, long, boolean)` constructor.
    pub fn new(
        id: i64,
        user_id: i32,
        item_id: i64,
        definition_id: i32,
        price: i32,
        lowest_price: i32,
        rotation: i32,
        state: i32,
        colour: i32,
        items_alike: i32,
        created_at: i64,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            item_id,
            definition_id,
            price,
            lowest_price,
            rotation,
            state,
            colour,
            created_at,
            is_active,
            items_alike,
        }
    }

    /// Mirrors `getTimeUntilExpiry()`.
    pub fn get_time_until_expiry(&self) -> Option<String> {
        let timestamp = self.created_at + 604800;
        DateUtil::get_marketplace_readable_seconds(timestamp - DateUtil::get_current_time_seconds() as i64)
    }

    /// Mirrors `getDefinition()`.
    pub fn get_definition(&self) -> Option<ItemDefinition> {
        ItemManager::get_instance().get_definition(self.definition_id)
    }

    /// Mirrors `getFormattedPrice()`.
    pub fn get_formatted_price(&self) -> String {
        Self::format_us_number(self.price)
    }

    /// Mirrors `getFormattedLowestPrice()`.
    pub fn get_formatted_lowest_price(&self) -> String {
        Self::format_us_number(self.lowest_price)
    }

    /// Mirrors `NumberFormat.getInstance(Locale.US).format(int)`.
    fn format_us_number(value: i32) -> String {
        let digits = (value.abs() as i64).to_string();
        let mut result = String::new();

        for (index, digit) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index) % 3 == 0 {
                result.push(',');
            }
            result.push(digit);
        }

        if value < 0 {
            format!("-{result}")
        } else {
            result
        }
    }
}
