//! Mirrors `net.h4bbo.lisbon.game.item.roller.RollerEntry`.
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::item::roller::rolling_data::RollingData;

pub struct RollerEntry {
    roller: Item,
    rolling_items: Vec<RollingData>,
    rolling_entity: Option<Box<dyn Entity + Send>>,
}

impl RollerEntry {
    /// Mirrors the `RollerEntry(Item)` constructor.
    pub fn new(roller: Item) -> Self {
        Self {
            roller,
            rolling_items: Vec::new(),
            rolling_entity: None,
        }
    }

    /// Mirrors `getRoller()`.
    pub fn get_roller(&self) -> &Item {
        &self.roller
    }

    /// Mirrors `getRollingItems()`.
    /// Mirrors `getRollingItems().add(RollingData)`.
    pub fn add_rolling_item(&mut self, data: RollingData) {
        self.rolling_items.push(data);
    }

    pub fn get_rolling_items(&self) -> &[RollingData] {
        &self.rolling_items
    }

    /// Mirrors `getRollingEntity()`.
    pub fn get_rolling_entity(&self) -> Option<&(dyn Entity + Send)> {
        self.rolling_entity
            .as_ref()
            .map(|entity| entity.as_ref())
    }

    /// Mirrors `setRollingEntity(Entity)`.
    pub fn set_rolling_entity(&mut self, rolling_entity: Option<Box<dyn Entity + Send>>) {
        self.rolling_entity = rolling_entity;
    }
}
