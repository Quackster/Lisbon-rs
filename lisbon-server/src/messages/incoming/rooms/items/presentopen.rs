//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.PRESENTOPEN`.
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::messages::outgoing::catalogue::deliver_present::DELIVER_PRESENT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c.is_ascii_digit())
}

#[allow(non_camel_case_types)]
pub struct PRESENTOPEN;

impl MessageEvent for PRESENTOPEN {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        // The Java throws on a non-numeric id.
        let Ok(item_id) = reader.contents().unwrap_or_default().parse::<i32>() else {
            return Ok(());
        };

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Present) {
            return Ok(());
        }

        let present_data: Vec<String> = item
            .get_custom_data()
            .split(Item::PRESENT_DELIMETER)
            .map(|part| part.to_string())
            .collect();

        // The Java `ArrayIndexOutOfBounds` when the present data is short.
        if present_data.len() < 5 {
            return Ok(());
        }

        let sale_code = present_data[0].clone();
        let received_from = present_data[1].clone();
        let extra_data = present_data[3].clone();

        // The Java `NumberFormatException` on a malformed timestamp.
        let Ok(timestamp) = present_data[4].parse::<i64>() else {
            return Ok(());
        };

        let catalogue_manager = CatalogueManager::get_instance();

        let catalogue_item = if is_numeric(&sale_code) {
            catalogue_manager
                .get_catalogue_items()
                .into_iter()
                .find(|shop_item| shop_item.get_id() == sale_code.parse::<i32>().unwrap_or(0))
        } else {
            catalogue_manager.get_catalogue_item(&sale_code)
        };

        let Some(catalogue_item) = catalogue_item else {
            // The Java NPEs when the catalogue item is missing.
            return Ok(());
        };

        // Don't create a new item instance, reuse if the item isn't a
        // trophy or teleporter, etc
        let Some(definition) = catalogue_item.get_definition() else {
            // The Java NPEs when the definition is missing.
            return Ok(());
        };

        if !catalogue_item.is_package()
            && !definition.has_behaviour(ItemBehaviour::PrizeTrophy)
            && !definition.has_behaviour(ItemBehaviour::Teleporter)
            && !definition.has_behaviour(ItemBehaviour::RoomDimmer)
            && !definition.has_behaviour(ItemBehaviour::Decoration)
            && !definition.has_behaviour(ItemBehaviour::PostIt)
            && !definition.get_sprite().eq_ignore_ascii_case("film")
        {
            room.get_mapping().lock().remove_item(&room, &mut item);

            item.set_definition_id(definition.get_id());
            item.set_custom_data(&extra_data);
            item.save();

            player.send(&DELIVER_PRESENT::new(
                definition.get_sprite(),
                &extra_data,
                definition.get_colour(),
            ));

            if let Some(inventory) = player.get_inventory() {
                inventory.add_item(&item);
                inventory.view(player, "new");
            }
        } else {
            let item_list = catalogue_manager.purchase(
                player.get_details(),
                catalogue_item,
                Some(&extra_data),
                Some(&received_from),
                timestamp,
            );

            if let Some(gifted_item) = item_list.first() {
                player.send(&DELIVER_PRESENT::new(
                    gifted_item.get_definition().get_sprite(),
                    &extra_data,
                    gifted_item.get_definition().get_colour(),
                ));

                if let Some(inventory) = player.get_inventory() {
                    inventory.view(player, "new");
                }
            } else {
                // item list will be blank if this was film purchased,
                // however, still show film when gift is opened
                if definition.get_sprite().eq_ignore_ascii_case("film") {
                    player.send(&DELIVER_PRESENT::new("film", "", ""));
                }
            }

            room.get_mapping().lock().remove_item(&room, &mut item);
            item.delete();
        }

        Ok(())
    }
}
