//! Mirrors `org.alexdev.http.game.news.NewsManager`.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::dao::news_dao::NewsDao;
use crate::game::news::news_category::NewsCategory;

pub struct NewsManager {
    news_category_map: HashMap<i32, NewsCategory>,
}

impl NewsManager {
    /// Mirrors the `NewsManager()` constructor.
    fn new() -> Self {
        Self {
            news_category_map: NewsDao::get_categories(),
        }
    }

    /// Mirrors `getCategoryById(int)`.
    pub fn get_category_by_id(&self, category_id: i32) -> Option<NewsCategory> {
        self.news_category_map.get(&category_id).cloned()
    }

    /// Mirrors `getCategoryByLabel(String)`.
    pub fn get_category_by_label(&self, category_label: &str) -> Option<NewsCategory> {
        self.news_category_map
            .values()
            .find(|category| category.index.eq_ignore_ascii_case(category_label))
            .cloned()
    }

    /// Mirrors `getCategories()`.
    pub fn get_categories(&self) -> Vec<NewsCategory> {
        let mut categories: Vec<NewsCategory> = self.news_category_map.values().cloned().collect();
        categories.sort_by(|left, right| left.label.cmp(&right.label));
        categories
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static NewsManager {
        static INSTANCE: OnceLock<NewsManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }
}
