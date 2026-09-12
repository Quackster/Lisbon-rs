//! Mirrors `net.h4bbo.lisbon.dao.mysql.TeleporterDao`.

use crate::dao::storage::{RowGetters, Storage};

pub struct TeleporterDao;

impl TeleporterDao {
    /// Mirrors `getTeleporterId(int)`.
    pub fn get_teleporter_id(item_id: i32) -> i32 {
        let mut teleporter_id = -1;

        for row in Storage::get_storage().query_all(
            &format!("SELECT linked_id FROM items_teleporter_links WHERE item_id = {item_id}"),
        ) {
            if let Some(id) = row.i32("linked_id") {
                teleporter_id = id;
            }
        }

        teleporter_id
    }

    /// Mirrors `addPair(int, int)`.
    pub fn add_pair(item_id: i32, linked_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO items_teleporter_links (item_id, linked_id) VALUES ({item_id}, {linked_id})"
        ));
    }
}
