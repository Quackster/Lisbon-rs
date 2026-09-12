//! Mirrors `org.alexdev.http.dao.StoreDao`.

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};

pub use crate::game::stickers::sticker_category::StickerCategory;
pub use crate::game::stickers::sticker_product::StickerProduct;

pub struct StoreDao;

impl StoreDao {
    /// Mirrors `getCategories()`.
    pub fn get_categories() -> Vec<StickerCategory> {
        let mut category_list = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM cms_stickers_categories") {
            category_list.push(Self::fill_category(&row));
        }

        category_list
    }

    /// Mirrors `getCatalogue()`.
    pub fn get_catalogue() -> Vec<StickerProduct> {
        let mut product_list = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM cms_stickers_catalogue") {
            product_list.push(Self::fill_product(&row));
        }

        product_list
    }

    /// Mirrors the `StickerCategory` constructor mapping.
    fn fill_category(row: &MySqlRow) -> StickerCategory {
        let name = row.str("name").unwrap_or_default();

        StickerCategory::new(
            row.i32("id").unwrap_or(0),
            &name,
            row.i32("min_rank").unwrap_or(0),
            row.i32("category_type").unwrap_or(0),
        )
    }

    /// Mirrors the `StickerProduct` constructor mapping.
    fn fill_product(row: &MySqlRow) -> StickerProduct {
        let name = row.str("name").unwrap_or_default();
        let description = row.str("description").unwrap_or_default();
        let data = row.str("data").unwrap_or_default();

        StickerProduct::new(
            row.i32("id").unwrap_or(0),
            &name,
            &description,
            row.i32("min_rank").unwrap_or(0),
            &data,
            row.i32("price").unwrap_or(0),
            row.i32("amount").unwrap_or(0),
            row.i32("category_id").unwrap_or(0),
            row.i32("widget_type").unwrap_or(0),
            row.i32("type").unwrap_or(0),
        )
    }
}
