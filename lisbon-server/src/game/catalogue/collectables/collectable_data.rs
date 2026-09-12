//! Mirrors `net.h4bbo.lisbon.game.catalogue.collectables.CollectableData`.

use crate::dao::mysql::collectables_dao::CollectablesDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug)]
pub struct CollectableData {
    current_position: usize,
    expiry: i64,
    lifetime: i64,
    collectables_store_page: i32,
    collectables_admin_page: i32,
    class_names: Vec<String>,
}

impl CollectableData {
    /// Mirrors the `CollectableData` constructor.
    pub fn new(
        collectables_store_page: i32,
        collectables_admin_page: i32,
        expiry: i64,
        lifetime: i64,
        current_position: i32,
        class_names: Vec<String>,
    ) -> Self {
        Self {
            current_position: current_position as usize,
            expiry,
            lifetime,
            collectables_store_page,
            collectables_admin_page,
            class_names,
        }
    }

    /// Mirrors `checkCycle()`.
    pub fn check_cycle(&mut self) {
        if !(DateUtil::get_current_time_seconds() as i64 > self.expiry) {
            return;
        }

        self.current_position += 1;

        if self.current_position >= self.class_names.len() {
            self.current_position = 0;
        }

        self.expiry = DateUtil::get_current_time_seconds() as i64 + self.lifetime;
        CollectablesDao::save_data(
            self.collectables_store_page,
            self.current_position as i32,
            self.expiry,
        );
    }

    /// Mirrors `getActiveItem()`.
    pub fn get_active_item(&self) -> Option<CatalogueItem> {
        if self.current_position >= self.class_names.len() {
            return None;
        }

        let class_name = &self.class_names[self.current_position];

        for item in CatalogueManager::get_instance()
            .get_catalogue_page_items(self.collectables_admin_page, true)
        {
            if let Some(definition) = item.get_definition() {
                if definition.get_sprite() == class_name {
                    return Some(item.copy());
                }
            }
        }

        None
    }

    /// Mirrors `getSprites()`.
    pub fn get_sprites(&self) -> Vec<String> {
        self.class_names.clone()
    }

    /// Mirrors `getExpiry()`.
    pub fn get_expiry(&self) -> i64 {
        self.expiry
    }

    /// Mirrors `getCollectablesStorePage()`.
    pub fn get_collectables_store_page(&self) -> i32 {
        self.collectables_store_page
    }

    /// Mirrors `getCollectablesAdminPage()`.
    pub fn get_collectables_admin_page(&self) -> i32 {
        self.collectables_admin_page
    }
}
