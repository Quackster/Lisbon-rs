//! Mirrors `net.h4bbo.lisbon.dao.mysql.RareDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct RareDao;

impl RareDao {
    /// Mirrors `addRare(String, long)`.
    pub fn add_rare(sprite: &str, reuse_time: i64) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO rare_cycle (sale_code, reuse_time) VALUES ('{}', {reuse_time})",
            escape(sprite)
        ));
    }

    /// Mirrors `removeRares(List<String>)`.
    pub fn remove_rares(sprites: Vec<String>) {
        for sprite in sprites {
            Storage::get_storage().execute(&format!(
                "DELETE FROM rare_cycle WHERE sale_code = '{}'",
                escape(&sprite)
            ));
        }
    }

    /// Mirrors `getUsedRares()`.
    pub fn get_used_rares() -> HashMap<String, i64> {
        let mut rares = HashMap::new();

        for row in Storage::get_storage().query_all(
            "SELECT sale_code, reuse_time FROM rare_cycle ORDER BY reuse_time DESC",
        ) {
            if let (Some(sale_code), Some(reuse_time)) = (row.str("sale_code"), row.i64("reuse_time")) {
                rares.insert(sale_code, reuse_time);
            }
        }

        rares
    }

    /// Mirrors `getCurrentRare()`.
    pub fn get_current_rare() -> Option<(String, i64)> {
        for row in Storage::get_storage().query_all(
            "SELECT sale_code, reuse_time FROM rare_cycle ORDER BY reuse_time DESC LIMIT 1",
        ) {
            if let (Some(sale_code), Some(reuse_time)) = (row.str("sale_code"), row.i64("reuse_time")) {
                return Some((sale_code, reuse_time));
            }
        }

        None
    }
}
