//! Mirrors `net.h4bbo.lisbon.game.navigator.NavigatorManager`.
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::dao::mysql::navigator_dao::NavigatorDao;
use crate::game::navigator::navigator_category::NavigatorCategory;

pub struct NavigatorManager {
    category_map: OnceLock<HashMap<i32, NavigatorCategory>>,
}

impl NavigatorManager {
    /// Get the instance.
    pub fn get_instance() -> &'static NavigatorManager {
        static INSTANCE: OnceLock<NavigatorManager> = OnceLock::new();
        INSTANCE.get_or_init(|| NavigatorManager {
            category_map: OnceLock::new(),
        })
    }

    fn categories(&self) -> &HashMap<i32, NavigatorCategory> {
        self.category_map.get_or_init(NavigatorDao::get_categories)
    }

    /// Mirrors `getCategoriesByParentId(int)`.
    pub fn get_categories_by_parent_id(&self, parent_id: i32) -> Vec<NavigatorCategory> {
        self.categories()
            .values()
            .filter(|category| category.get_parent_id() == parent_id)
            .cloned()
            .collect()
    }

    /// Mirrors `getCategoryById(int)`.
    pub fn get_category_by_id(&self, category_id: i32) -> Option<NavigatorCategory> {
        self.categories().get(&category_id).cloned()
    }

    /// Mirrors `getCategories()`.
    pub fn get_categories(&self) -> HashMap<i32, NavigatorCategory> {
        self.categories().clone()
    }
}
