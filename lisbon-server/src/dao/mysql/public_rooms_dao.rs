//! Mirrors `net.h4bbo.lisbon.dao.mysql.PublicRoomsDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::item::public_items::public_item_data::PublicItemData;
use crate::game::room::handlers::walkways::walkways_entrance::WalkwaysEntrance;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;

pub struct PublicRoomsDao;

impl PublicRoomsDao {
    /// Mirrors `getPublicItemData(String)`.
    pub fn get_public_item_data(room_model: &str) -> Vec<PublicItemData> {
        let mut item_data_list = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM public_items WHERE room_model = '{}'",
                room_model.replace('\'', "''")
            ),
        ) {
            if let (Some(id), Some(room_model), Some(sprite), Some(x), Some(y), Some(z), Some(rotation), Some(top_height), Some(length), Some(width), Some(behaviour), Some(current_program), teleport_to, swim_to) = (
                row.str("id"),
                row.str("room_model"),
                row.str("sprite"),
                row.i32("x"),
                row.i32("y"),
                row.f64("z"),
                row.i32("rotation"),
                row.f64("top_height"),
                row.i32("length"),
                row.i32("width"),
                row.str("behaviour"),
                row.str("current_program"),
                row.str("teleport_to"),
                row.str("swim_to"),
            ) {
                item_data_list.push(PublicItemData::new(
                    &id,
                    &room_model,
                    &sprite,
                    x,
                    y,
                    z,
                    rotation,
                    top_height,
                    length,
                    width,
                    &behaviour,
                    &current_program,
                    teleport_to.as_deref(),
                    swim_to.as_deref(),
                ));
            }
        }

        item_data_list
    }

    /// Mirrors `getWalkways()`.
    pub fn get_walkways() -> Vec<WalkwaysEntrance> {
        let mut walkways = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM public_roomwalkways") {
            if let (Some(room_id), Some(to_id), Some(coords_map)) =
                (row.i32("room_id"), row.i32("to_id"), row.str("coords_map"))
            {
                walkways.push(WalkwaysManager::create_walkway(
                    room_id,
                    to_id,
                    &coords_map,
                    row.str("door_position").as_deref(),
                ));
            }
        }

        walkways
    }
}
