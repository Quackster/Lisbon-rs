//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.mapping.SnowStormTile`.
use std::sync::Arc;

use crate::game::games::snowstorm::mapping::snow_storm_item::SnowStormItem;
use crate::game::games::snowstorm::objects::snowball_object::SnowballTrajectory;

#[derive(Clone, Debug)]
pub struct SnowStormTile {
    x: i32,
    y: i32,
    items: Vec<Arc<SnowStormItem>>,
    highest_item: Option<Arc<SnowStormItem>>,
    is_blocked: bool,
}

impl SnowStormTile {
    /// Mirrors the `SnowStormTile(int, int, boolean, List<SnowStormItem>)`
    /// constructor.
    pub fn new(
        x: i32,
        y: i32,
        is_blocked: bool,
        items: Vec<Arc<SnowStormItem>>,
    ) -> Self {
        let mut items = items;
        items.sort_by(|a, b| a.get_z().cmp(&b.get_z()));

        let mut highest_item: Option<Arc<SnowStormItem>> = None;
        for item in &items {
            if highest_item
                .as_ref()
                .map(|h| item.get_z() > h.get_z())
                .unwrap_or(true)
            {
                highest_item = Some(Arc::clone(item));
            }
        }

        Self {
            x,
            y,
            items,
            highest_item,
            is_blocked,
        }
    }

    /// Mirrors `isWalkable()`.
    pub fn is_walkable(&self) -> bool {
        if self.is_blocked {
            return false;
        }

        if self.highest_item.is_some() {
            return false;
        }

        true
    }

    /// Mirrors `getHighestItem()`.
    pub fn get_highest_item(&self) -> Option<&Arc<SnowStormItem>> {
        self.highest_item.as_ref()
    }

    /// Mirrors `getItems()`.
    pub fn get_items(&self) -> &Vec<Arc<SnowStormItem>> {
        &self.items
    }

    /// Mirrors `isHeightBlocking(SnowballObject.SnowballTrajectory)`.
    pub fn is_height_blocking(&self, trajectory: Option<SnowballTrajectory>) -> bool {
        let Some(highest_item) = self.highest_item.as_ref() else {
            return false;
        };

        match trajectory {
            Some(SnowballTrajectory::LongTrajectory) => false,
            Some(SnowballTrajectory::ShortTrajectory) => highest_item.get_height() > 1,
            Some(SnowballTrajectory::QuickThrow) => highest_item.get_height() > 0,
            None => false,
        }
    }
}
