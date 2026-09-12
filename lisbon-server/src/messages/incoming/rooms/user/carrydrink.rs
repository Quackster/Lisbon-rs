//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.CARRYDRINK`.
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CARRYDRINK;

impl MessageEvent for CARRYDRINK {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if room.is_public_room() {
            let contents = reader.contents().unwrap_or_default();

            if !contents.is_empty() && contents.chars().all(|c| c.is_ascii_digit()) {
                let front = room_user.get_position().get_square_in_front();

                let tile_has_teamk = room
                    .get_mapping().lock()
                    .get_tile(&room, front.get_x(), front.get_y())
                    .map_or(false, |room_tile| {
                        room_tile
                            .get_items()
                            .iter()
                            .any(|item| item.get_definition().get_sprite() == "arabian_teamk")
                    });

                if tile_has_teamk {
                    room_user.carry_item(1, None);
                } else {
                    room_user.carry_item(contents.parse::<i32>().unwrap_or(0), None);
                }
            } else {
                room_user.carry_item(-1, Some(&contents));
            }
        } else {
            if room_user.get_last_item_interaction().is_none() {
                let front = room_user.get_position().get_square_in_front();

                let vending = room
                    .get_mapping().lock()
                    .get_tile(&room, front.get_x(), front.get_y())
                    .and_then(|room_tile| {
                        room_tile
                            .get_items()
                            .into_iter()
                            .find(|item| {
                                item.get_definition().get_interaction_type()
                                    == Some(InteractionType::VendingMachine)
                            })
                            .cloned()
                    });

                if let Some(vending) = vending {
                    room_user.set_last_item_interaction(&vending);
                }

                if room_user.get_last_item_interaction().is_none() {
                    return Ok(());
                }
            }

            let Some(item) = room_user.get_last_item_interaction() else {
                return Ok(());
            };

            if item
                .get_definition()
                .get_interaction_type()
                != Some(InteractionType::VendingMachine)
            {
                return Ok(());
            }

            // The `Item.getTile()` port is pending (a `MutexGuard` cannot be
            // returned across the room lock), so the tile is resolved via the
            // shared tile handle on the item's room.
            let item_tile_pos = item
                .get_room()
                .map(|room_arc| {
                    let r = room_arc.lock();
                    let pos = item.get_position();
                    r.get_mapping().lock()
                        .get_tile_handle_for(&r, pos.get_x(), pos.get_y())
                        .map(|t| t.lock().get_position().copy())
                })
                .flatten();

            let user_tile_pos = room_user
                .get_tile()
                .map(|t| t.lock().get_position().copy());

            let touches = match (item_tile_pos, user_tile_pos) {
                (Some(a), Some(b)) => a.touches(&b),
                _ => false,
            };

            if !touches {
                return Ok(());
            }

            let drink_ids = item.get_definition().get_drink_ids().to_vec();
            let random_drink_id = if drink_ids.is_empty() {
                0
            } else {
                drink_ids[
                    rand::thread_rng().gen_range(0..drink_ids.len())
                ]
            };

            room_user.carry_item(random_drink_id, None);
            room_user.clear_last_item_interaction();
        }

        room_user.reset_room_timer();

        Ok(())
    }
}
