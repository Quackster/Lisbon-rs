//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomModelDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::room::models::room_model::RoomModel;

pub struct RoomModelDao;

impl RoomModelDao {
    /// Mirrors `getModels()` (the Java `ConcurrentHashMap<String, RoomModel>`).
    pub fn get_models() -> HashMap<String, RoomModel> {
        let mut room_models: HashMap<String, RoomModel> = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM rooms_models") {
            if let (Some(model_id), Some(model_name), Some(door_x), Some(door_y), Some(door_z), Some(door_dir), Some(heightmap), trigger_class) = (
                row.str("model_id"),
                row.str("model_name"),
                row.i32("door_x"),
                row.i32("door_y"),
                row.f64("door_z"),
                row.i32("door_dir"),
                row.str("heightmap"),
                row.str("trigger_class"),
            ) {
                let model = RoomModel::new(
                    &model_id,
                    &model_name,
                    door_x,
                    door_y,
                    door_z,
                    door_dir,
                    &heightmap,
                    trigger_class.as_deref(),
                );

                room_models.insert(model.get_id().to_string(), model);
            }
        }

        room_models
    }
}
