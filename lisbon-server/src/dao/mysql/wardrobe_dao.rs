//! Mirrors `net.h4bbo.lisbon.dao.mysql.WardrobeDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::player::wardrobe::Wardrobe;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct WardrobeDao;

impl WardrobeDao {
    /// Mirrors `getWardrobe(int)`.
    pub fn get_wardrobe(user_id: i32) -> Vec<Wardrobe> {
        let mut wardrobe_list = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM users_wardrobes WHERE user_id = {user_id}"))
        {
            if let (Some(slot_id), Some(sex), Some(figure)) = (
                row.i32("slot_id"),
                row.str("sex"),
                row.str("figure"),
            ) {
                wardrobe_list.push(Wardrobe::new(slot_id, &sex, &figure));
            }
        }

        wardrobe_list
    }

    /// Mirrors `deleteWardrobe(int, int)`.
    pub fn delete_wardrobe(user_id: i32, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_wardrobes WHERE user_id = {user_id} AND slot_id = {slot_id}"
        ));
    }

    /// Mirrors `addWardrobe(int, int, String, String)`.
    pub fn add_wardrobe(user_id: i32, slot_id: i32, figure: &str, sex: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_wardrobes (user_id, slot_id, figure, sex) VALUES ({user_id}, {slot_id}, '{}', '{}')",
            escape(figure),
            escape(sex)
        ));
    }

    /// Mirrors `updateWardrobe(int, int, String, String)`.
    pub fn update_wardrobe(user_id: i32, slot_id: i32, figure: &str, sex: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_wardrobes SET figure = '{}', sex = '{}' WHERE slot_id = {slot_id} AND user_id = {user_id}",
            escape(figure),
            escape(sex)
        ));
    }
}
