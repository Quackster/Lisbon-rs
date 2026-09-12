//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.HEIGHTMAP_UPDATE`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::room::models::room_model::RoomModel;
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct HEIGHTMAP_UPDATE {
    heightmap: String,
}

impl HEIGHTMAP_UPDATE {
    /// Mirrors the `HEIGHTMAP_UPDATE(Room, RoomModel)` constructor.
    pub fn new(room: &Room, room_model: &RoomModel) -> Self {
        let lines: Vec<&str> = room_model.get_heightmap().split('\r').collect();
        let mut update_map = String::new();

        for y in 0..room_model.get_map_size_y() {
            let line = lines.get(y as usize).copied().unwrap_or("");

            for x in 0..room_model.get_map_size_x() {
                match line.chars().nth(x as usize) {
                    Some(tile) if tile.is_ascii_digit() => {
                        let mut height: i32 = 0;

                        if let Some(tile) = room.get_mapping().lock().get_tile(room, x, y) {
                            height = tile.get_walking_height() as i32;

                            if height < 0 || height > 8 {
                                height = 0;
                            }

                            if let Some(highest_item) = tile.get_highest_item() {
                                if highest_item.has_behaviour(ItemBehaviour::CanStackOnTop) {
                                    // The clamped height fits in `u8` here.
                                    update_map
                                        .push(char::from_u32(65 + height as u32).unwrap());
                                    continue;
                                }
                            }
                        } else {
                            // The Java NPEs when the tile is missing from
                            // the mapping.
                        }

                        update_map.push_str(&height.to_string());
                    }
                    _ => update_map.push('x'),
                }
            }

            update_map.push('\r');
        }

        Self {
            heightmap: update_map,
        }
    }
}

impl MessageComposer for HEIGHTMAP_UPDATE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.heightmap.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        219 // "@_"
    }
}
