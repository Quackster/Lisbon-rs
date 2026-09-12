//! Mirrors `net.h4bbo.lisbon.dao.mysql.MoodlightDao`.

use crate::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct MoodlightDao;

impl MoodlightDao {
    /// Mirrors `containsPreset(int)`.
    pub fn contains_preset(item_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(
                &format!("SELECT item_id FROM items_moodlight_presets WHERE item_id = {item_id}"),
            )
        {
            return true;
        }

        false
    }

    /// Mirrors `createPresets(int)`.
    pub fn create_presets(item_id: i32) -> bool {
        Storage::get_storage().execute(&format!(
            "INSERT INTO items_moodlight_presets (item_id) VALUES ({item_id})"
        ));

        false
    }

    /// Mirrors `updatePresets(int, int, List<String>)`.
    pub fn update_presets(item_id: i32, current_preset: i32, preset_data: &[String]) -> bool {
        Storage::get_storage().execute(&format!(
            "UPDATE items_moodlight_presets SET current_preset = {current_preset}, preset_1 = '{}', preset_2 = '{}', preset_3 = '{}' WHERE item_id = {item_id}",
            escape(&preset_data[0]),
            escape(&preset_data[1]),
            escape(&preset_data[2])
        ));

        false
    }

    /// Mirrors `deletePresets(int)`.
    pub fn delete_presets(item_id: i32) -> bool {
        Storage::get_storage().execute(&format!(
            "DELETE FROM items_moodlight_presets WHERE item_id = {item_id}"
        ));

        false
    }

    /// Mirrors `getPresets(int)` (the Java `Pair<Integer, ArrayList<String>>`).
    pub fn get_presets(item_id: i32) -> Option<(i32, Vec<String>)> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM items_moodlight_presets WHERE item_id = {item_id} LIMIT 1"))
        {
            if let (Some(current_preset), preset_1, preset_2, preset_3) = (
                row.i32("current_preset"),
                row.str("preset_1"),
                row.str("preset_2"),
                row.str("preset_3"),
            ) {
                let mut presets = Vec::new();
                presets.push(preset_1.unwrap_or_default());
                presets.push(preset_2.unwrap_or_default());
                presets.push(preset_3.unwrap_or_default());

                return Some((current_preset, presets));
            }
        }

        None
    }
}
