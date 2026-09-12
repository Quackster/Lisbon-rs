//! Mirrors `net.h4bbo.lisbon.dao.mysql.CatalogueDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_page::CataloguePage;
use crate::game::catalogue::catalogue_package::CataloguePackage;
use crate::game::player::player_rank::PlayerRank;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct CatalogueDao;

impl CatalogueDao {
    /// Mirrors `getPages()`.
    pub fn get_pages() -> Vec<CataloguePage> {
        let mut pages = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM catalogue_pages ORDER BY order_id ASC") {
            if let (Some(id), Some(min_role), Some(index_visible), Some(is_club_only), Some(name_index), Some(link_list), Some(name), Some(layout), Some(image_headline), image_teasers, body, Some(label_pick), Some(label_extra_s), label_extra_t) = (
                row.i32("id"),
                row.i32("min_role"),
                row.bool("index_visible"),
                row.bool("is_club_only"),
                row.str("name_index"),
                row.str("link_list"),
                row.str("name"),
                row.str("layout"),
                row.str("image_headline"),
                row.str("image_teasers"),
                row.str("body"),
                row.str("label_pick"),
                row.str("label_extra_s"),
                row.str("label_extra_t"),
            ) {
                pages.push(CataloguePage::new(
                    id,
                    PlayerRank::get_rank_for_id(min_role).unwrap_or(PlayerRank::Rankless),
                    index_visible,
                    is_club_only,
                    &name_index,
                    &link_list,
                    &name,
                    &layout,
                    &image_headline,
                    image_teasers.as_deref(),
                    body.as_deref(),
                    &label_pick,
                    &label_extra_s,
                    label_extra_t.as_deref(),
                ));
            }
        }

        pages
    }

    /// Mirrors `getItems()`.
    pub fn get_items() -> Vec<CatalogueItem> {
        let mut items = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM catalogue_items ORDER BY order_id ASC") {
            if let (Some(id), Some(sale_code), Some(page_id), Some(order_id), Some(price), Some(is_hidden), Some(definition_id), Some(item_special_id), Some(name), Some(description), Some(is_package), Some(package_name), Some(package_description)) = (
                row.i32("id"),
                row.str("sale_code"),
                row.str("page_id"),
                row.i32("order_id"),
                row.i32("price"),
                row.bool("is_hidden"),
                row.i32("definition_id"),
                row.i32("item_specialspriteid"),
                row.str("name"),
                row.str("description"),
                row.bool("is_package"),
                row.str("package_name"),
                row.str("package_description"),
            ) {
                items.push(CatalogueItem::new(
                    id,
                    &sale_code,
                    &page_id,
                    order_id,
                    price,
                    is_hidden,
                    definition_id,
                    item_special_id,
                    &name,
                    &description,
                    is_package,
                    &package_name,
                    &package_description,
                ));
            }
        }

        items
    }

    /// Mirrors `getPackages()`.
    pub fn get_packages() -> Vec<CataloguePackage> {
        let mut packages = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM catalogue_packages") {
            if let (Some(sale_code), Some(definition_id), Some(special_sprite_id), Some(amount)) = (
                row.str("salecode"),
                row.i32("definition_id"),
                row.i32("special_sprite_id"),
                row.i32("amount"),
            ) {
                packages.push(CataloguePackage::new(
                    &sale_code,
                    definition_id,
                    special_sprite_id,
                    amount,
                ));
            }
        }

        packages
    }

    /// Mirrors `setPrice(String, int)`.
    pub fn set_price(sale_code: &str, price: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE catalogue_items SET price = {price} WHERE sale_code = '{}'",
            escape(sale_code)
        ));
    }
}
