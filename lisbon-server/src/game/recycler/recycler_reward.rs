//! Mirrors `net.h4bbo.lisbon.game.recycler.RecyclerReward`.

use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_manager::CatalogueManager;

#[derive(Clone, Debug)]
pub struct RecyclerReward {
    id: i32,
    sale_code: String,
    item_cost: i32,
    recycling_time_sessions: i32,
    collection_time_seconds: i32,
}

impl RecyclerReward {
    /// Mirrors the 5-arg `RecyclerReward(int, String, int, int, int)` constructor.
    pub fn new(
        id: i32,
        sale_code: String,
        item_cost: i32,
        recycling_time_sessions: i32,
        collection_time_seconds: i32,
    ) -> Self {
        Self {
            id,
            sale_code,
            item_cost,
            recycling_time_sessions,
            collection_time_seconds,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getSaleCode()`.
    pub fn get_sale_code(&self) -> &str {
        &self.sale_code
    }

    /// Mirrors `getCatalogueItem()`.
    pub fn get_catalogue_item(&self) -> Option<CatalogueItem> {
        CatalogueManager::get_instance()
            .get_catalogue_item(&self.sale_code)
    }

    /// Mirrors `getItemCost()`.
    pub fn get_item_cost(&self) -> i32 {
        self.item_cost
    }

    /// Mirrors `getRecyclingTimeSessions()`.
    pub fn get_recycling_time_sessions(&self) -> i32 {
        self.recycling_time_sessions
    }

    /// Mirrors `getCollectionTimeSeconds()`.
    pub fn get_collection_time_seconds(&self) -> i32 {
        self.collection_time_seconds
    }
}
