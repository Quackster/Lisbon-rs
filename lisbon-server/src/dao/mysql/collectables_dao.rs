//! Mirrors `net.h4bbo.lisbon.dao.mysql.CollectablesDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::catalogue::collectables::collectable_data::CollectableData;

pub struct CollectablesDao;

impl CollectablesDao {
    /// Mirrors `getCollectablesData()`.
    pub fn get_collectables_data() -> Vec<CollectableData> {
        let mut collectable_data = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM catalogue_collectables") {
            if let (Some(store_page), Some(admin_page), Some(expiry), Some(lifetime), Some(current_position), Some(class_names)) = (
                row.i32("store_page"),
                row.i32("admin_page"),
                row.i64("expiry"),
                row.i64("lifetime"),
                row.i32("current_position"),
                row.str("class_names"),
            ) {
                let names: Vec<String> = class_names.split(',').map(String::from).collect();
                collectable_data.push(CollectableData::new(
                    store_page,
                    admin_page,
                    expiry,
                    lifetime,
                    current_position,
                    names,
                ));
            }
        }

        collectable_data
    }

    /// Mirrors `saveData(int, int, long)`.
    pub fn save_data(store_page: i32, current_position: i32, expiry: i64) {
        Storage::get_storage().execute(&format!(
            "UPDATE catalogue_collectables SET expiry = {expiry}, current_position = {current_position} WHERE store_page = {store_page}"
        ));
    }
}
