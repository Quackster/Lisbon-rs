//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.PLACESTUFF`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::transaction_dao::TransactionDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c.is_ascii_digit())
}

#[allow(non_camel_case_types)]
pub struct PLACESTUFF;

impl MessageEvent for PLACESTUFF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split(' ').collect();

        if data.is_empty() {
            return Ok(());
        }

        // Make sure provided data is numeric
        if !is_numeric(data[0]) {
            return Ok(());
        }

        let item_id = data[0].parse::<i32>().unwrap_or(0);

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        let Some(mut item) = inventory.get_item(item_id) else {
            return Ok(());
        };

        if item.has_behaviour(ItemBehaviour::WallItem) {
            // The Java `StringIndexOutOfBounds` when the wall position is
            // missing.
            let Some(wall_position) = contents.get(data[0].len() + 1..) else {
                return Ok(());
            };

            if item.has_behaviour(ItemBehaviour::PostIt) {
                let default_colour = "FFFF33";

                let mut sticky = Item::new();
                sticky.set_owner_id(room.get_data().get_owner_id());
                sticky.set_definition_id(item.get_definition().get_id());
                sticky.set_custom_data(default_colour);
                sticky.set_wall_position(wall_position);
                sticky.set_room_id(room.get_id());

                ItemDao::new_item(&mut sticky);
                room.get_mapping().lock().add_item(&room, player, &mut sticky);

                // Set custom data as 1 for 1 post-it, if for some reason
                // they have no number for the post-it.
                if !is_numeric(item.get_custom_data()) {
                    item.set_custom_data("1");
                }

                if is_numeric(item.get_custom_data()) {
                    let total_stickies = item.get_custom_data().parse::<i32>().unwrap_or(0) - 1;

                    if total_stickies <= 0 {
                        inventory.remove_item(&item);
                        item.delete();
                    } else {
                        item.set_custom_data(&total_stickies.to_string());
                        item.save();
                    }
                }

                return Ok(());
            }

            item.set_wall_position(wall_position);
        } else {
            // The Java `NumberFormatException` on a malformed coordinate.
            let Some(x) = data.get(1).and_then(|value| value.parse::<i32>().ok()) else {
                return Ok(());
            };
            let Some(y) = data.get(2).and_then(|value| value.parse::<i32>().ok()) else {
                return Ok(());
            };

            // skip 3 and 4 as they're dimensions, we don't need 'em since
            // it's server-side variables, never trust the client!
            let Some(mut rotation) = data
                .get(3)
                .and_then(|value| value.parse::<i32>().ok())
            else {
                return Ok(());
            };

            if item.has_behaviour(ItemBehaviour::RedirectRotation0) {
                rotation = 0;
            }

            if item.has_behaviour(ItemBehaviour::RedirectRotation2) {
                rotation = 2;
            }

            if item.has_behaviour(ItemBehaviour::RedirectRotation4) {
                rotation = 4;
            }

            if !item.is_valid_move(&item, &room, Some(player), x, y, rotation) {
                return Ok(());
            }

            if room.get_mapping().lock().get_tile(&room, x, y).is_some() {
                let mut position = item.get_position().copy();
                position.set_x(x);
                position.set_y(y);
                position.set_rotation(rotation);
                item.set_position(position);
            }
        }

        if room
            .get_item_manager()
            .get_sound_machine()
            .is_some()
            && (item.has_behaviour(ItemBehaviour::SoundMachine)
                || item.has_behaviour(ItemBehaviour::Jukebox))
        {
            player.send(&ALERT::new(
                &TextsManager::get_instance().get_value("room_sound_furni_limit"),
            ));
            return Ok(());
        }

        if room.get_item_manager().get_moodlight().is_some()
            && item.has_behaviour(ItemBehaviour::RoomDimmer)
        {
            player.send(&ALERT::new(
                &TextsManager::get_instance().get_value("roomdimmer_furni_limit"),
            ));
            return Ok(());
        }

        if room.get_data().get_owner_id() != player.get_details().get_id() {
            TransactionDao::create_transaction(
                player.get_details().get_id(),
                &item.get_id().to_string(),
                &item.get_definition().get_id().to_string(),
                1,
                &format!(
                    "Placed item {} into {}'s room: {}",
                    item.get_definition().get_name(),
                    room.get_data().get_owner_name(),
                    room.get_id()
                ),
                room.get_id(),
                room.get_data().get_owner_id(),
                false,
            );
        }

        room.get_mapping().lock().add_item(&room, player, &mut item);
        inventory.remove_item(&item);

        Ok(())
    }
}
