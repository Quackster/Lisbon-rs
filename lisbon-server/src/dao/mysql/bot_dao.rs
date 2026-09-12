//! Mirrors `net.h4bbo.lisbon.dao.mysql.BotDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::bot::bot_data::BotData;

pub struct BotDao;

impl BotDao {
    /// Mirrors `getBotData(int)`.
    pub fn get_bot_data(room_id: i32) -> Vec<BotData> {
        let mut bot_data = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM rooms_bots WHERE room_id = {room_id}"),
        ) {
            if let (Some(name), Some(mission), Some(x), Some(y), Some(start_look), Some(figure), Some(walkspace), Some(speech), Some(response), Some(unrecognised_response), Some(hand_items)) = (
                row.str("name"),
                row.str("mission"),
                row.i32("x"),
                row.i32("y"),
                row.str("start_look"),
                row.str("figure"),
                row.str("walkspace"),
                row.str("speech"),
                row.str("response"),
                row.str("unrecognised_response"),
                row.str("hand_items"),
            ) {
                let parts: Vec<&str> = start_look.split(',').collect();
                let head_rotation = parts.get(0).and_then(|v| v.parse().ok()).unwrap_or(0);
                let body_rotation = parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);

                bot_data.push(BotData::new(
                    &name,
                    &mission,
                    x,
                    y,
                    head_rotation,
                    body_rotation,
                    &figure,
                    &walkspace,
                    &speech,
                    &response,
                    &unrecognised_response,
                    &hand_items,
                ));
            }
        }

        bot_data
    }
}
