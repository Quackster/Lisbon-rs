//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.SETSTUFFDATA`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SETSTUFFDATA;

impl MessageEvent for SETSTUFFDATA {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        // Do not process public room items.
        if room.is_public_room() {
            return Ok(());
        }

        let item_id = reader.read_string().parse::<i32>().unwrap_or(0);
        let item_data = reader.read_string();

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if item.has_behaviour(ItemBehaviour::RoomDimmer)
            || item.has_behaviour(ItemBehaviour::Dice)
            || item.has_behaviour(ItemBehaviour::PrizeTrophy)
            || item.has_behaviour(ItemBehaviour::PostIt)
            || item.has_behaviour(ItemBehaviour::Roller)
            || item.has_behaviour(ItemBehaviour::WheelOfFortune)
            || item.has_behaviour(ItemBehaviour::SoundMachineSampleSet)
        {
            return Ok(());
        }

        if item.has_behaviour(ItemBehaviour::Teleporter) {
            return Ok(());
        }

        if item.has_behaviour(ItemBehaviour::RequiresRightsForInteraction)
            && !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        // Item tile via the mapping handle (Item::get_tile is a stub).
        let item_tile_handle = room.get_mapping().lock().get_tile_handle_for(
            &room,
            item.get_position().get_x(),
            item.get_position().get_y(),
        );

        if item.has_behaviour(ItemBehaviour::RequiresTouchingForInteraction) {
            let touches = match (&item_tile_handle, room_user.get_tile()) {
                (Some(item_tile), Some(user_tile)) => {
                    let item_tile = item_tile.lock();
                    let user_tile = user_tile.lock();
                    item_tile.get_position().touches(user_tile.get_position())
                }
                _ => false,
            };

            if !touches {
                let mut next_position = item.get_position().get_square_in_front();

                if !item.has_behaviour(ItemBehaviour::Teleporter) {
                    if !RoomTile::is_valid_tile(&room, Some(player), &next_position) {
                        if let Some(item_tile) = &item_tile_handle {
                            let item_tile = item_tile.lock();
                            if let Some(available) =
                                item_tile.get_next_available_position(&room, Some(player))
                            {
                                next_position = available;
                            }
                        }
                    }

                    room_user.walk_to(next_position.get_x(), next_position.get_y());
                    return Ok(());
                }
            }
        }

        let mut new_data: Option<String> = None;

        if item.has_behaviour(ItemBehaviour::Gate) {
            if item_data == "O" || item_data == "C" {
                new_data = Some(item_data.clone());

                if item_data == "C" {
                    if let Some(item_tile) = &item_tile_handle {
                        let item_tile = item_tile.lock();

                        // Can't close the gate if there is a user on the tile.
                        if !item_tile.get_entities().is_empty() {
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            if item.has_behaviour(ItemBehaviour::CustomDataTrueFalse)
                && (item_data == "TRUE"
                    || item_data == "FALSE"
                    || item_data == "I"
                    || item_data == "H")
            {
                new_data = Some(item_data.clone());
            }

            if item.has_behaviour(ItemBehaviour::CustomDataNumericOnOff)
                && (item_data == "2" || item_data == "1" || item_data == "0")
            {
                new_data = Some(item_data.clone());
            }

            if item.has_behaviour(ItemBehaviour::CustomDataOnOff)
                && (item_data == "ON" || item_data == "OFF")
            {
                new_data = Some(item_data.clone());
            }

            if item.get_definition().get_sprite().starts_with("waterbowl") && item_data == "5" {
                new_data = Some(item_data.clone());
            }

            if item.has_behaviour(ItemBehaviour::CustomDataNumericState) {
                if item_data == "x" {
                    new_data = Some(item_data.clone());
                } else if !item_data.is_empty() && item_data.chars().all(|c| c.is_ascii_digit()) {
                    let state_id = item_data.parse::<i32>().unwrap_or(-1);

                    if state_id >= 0 && state_id <= 99 {
                        new_data = Some(item_data.clone());
                    }
                }
            }
        }

        let Some(new_data) = new_data else {
            return Ok(());
        };

        item.set_custom_data(&new_data);
        item.update_status();
        room_user.set_last_item_interaction(&item);

        if !item.get_definition().has_behaviour(ItemBehaviour::CustomDataTrueFalse) {
            item.save();
        }

        Ok(())
    }
}
