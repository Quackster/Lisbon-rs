//! Mirrors `net.h4bbo.lisbon.game.catalogue.CataloguePackage`.

use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::item_manager::ItemManager;

#[derive(Clone, Debug, serde::Serialize)]
pub struct CataloguePackage {
    sale_code: String,
    definition_id: i32,
    special_sprite_id: i32,
    amount: i32,
}

impl CataloguePackage {
    /// Mirrors the `CataloguePackage` constructor.
    pub fn new(sale_code: &str, definition_id: i32, special_sprite_id: i32, amount: i32) -> Self {
        Self {
            sale_code: sale_code.to_string(),
            definition_id,
            special_sprite_id,
            amount,
        }
    }

    /// Mirrors `getSaleCode()`.
    pub fn get_sale_code(&self) -> &str {
        &self.sale_code
    }

    /// Mirrors `getDefinitionId()`.
    pub fn get_definition_id(&self) -> i32 {
        self.definition_id
    }

    /// Mirrors `getDefinition()`.
    pub fn get_definition(&self) -> Option<ItemDefinition> {
        ItemManager::get_instance().get_definition(self.definition_id)
    }

    /// Mirrors `getSpecialSpriteId()`.
    pub fn get_special_sprite_id(&self) -> i32 {
        self.special_sprite_id
    }

    /// Mirrors `getAmount()`.
    pub fn get_amount(&self) -> i32 {
        self.amount
    }
}
