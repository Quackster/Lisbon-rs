//! Mirrors `net.h4bbo.lisbon.game.catalogue.collectables.CollectablesManager`.

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};

use crate::dao::mysql::collectables_dao::CollectablesDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::collectables::collectable_data::CollectableData;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<CollectablesManager>>> = RwLock::new(None);
}

pub struct CollectablesManager {
    collectable_data_list: Mutex<Vec<CollectableData>>,
}

impl CollectablesManager {
    fn new() -> Self {
        let collectable_data_list = CollectablesDao::get_collectables_data();

        Self {
            collectable_data_list: Mutex::new(collectable_data_list),
        }
    }

    /// Mirrors `checkExpiries()`.
    pub fn check_expiries(&self) {
        for collectable_data in self.collectable_data_list.lock().iter_mut() {
            collectable_data.check_cycle();
        }
    }

    /// Mirrors `isCollectable(CatalogueItem)`.
    pub fn is_collectable(&self, item: &CatalogueItem) -> bool {
        for collectable_data in self.collectable_data_list.lock().iter() {
            if let Some(collectable_item) = collectable_data.get_active_item() {
                if collectable_item.get_id() == item.get_id() {
                    return true;
                }
            }
        }

        false
    }

    /// Mirrors `getCollectableDataByPage(int)`.
    pub fn get_collectable_data_by_page(&self, page_id: i32) -> Option<CollectableData> {
        for collectable_data in self.collectable_data_list.lock().iter() {
            if collectable_data.get_collectables_store_page() == page_id
                || collectable_data.get_collectables_admin_page() == page_id
            {
                return Some(collectable_data.clone());
            }
        }

        None
    }

    /// Mirrors `getCollectableDataByItem(int)`.
    pub fn get_collectable_data_by_item(&self, item_id: i32) -> Option<CollectableData> {
        for collectable_data in self.collectable_data_list.lock().iter() {
            if let Some(active_item) = collectable_data.get_active_item() {
                if active_item.get_id() == item_id {
                    return Some(collectable_data.clone());
                }
            }
        }

        None
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<CollectablesManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
