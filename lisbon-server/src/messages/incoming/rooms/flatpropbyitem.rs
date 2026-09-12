//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.FLATPROPBYITEM`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::flatproperty::FLATPROPERTY;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct FLATPROPBYITEM;

impl MessageEvent for FLATPROPBYITEM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(mut room) = player
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

        let contents = reader.contents().unwrap_or_default();
        let parts: Vec<&str> = contents.split('/').collect();

        // The Java `ArrayIndexOutOfBounds` when the item id is missing.
        if parts.len() < 2 {
            return Ok(());
        }

        let property = parts[0];

        // The Java `NumberFormatException` on a malformed item id.
        let Ok(item_id) = parts[1].parse::<i32>() else {
            return Ok(());
        };

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        let Some(item) = inventory.get_item(item_id) else {
            return Ok(());
        };

        //int value = Integer.parseInt(item.getCustomData());

        if property == "wallpaper" {
            // The Java `NumberFormatException` on a malformed value.
            let Ok(value) = item.get_custom_data().parse::<i32>() else {
                return Ok(());
            };
            room.get_data_mut().set_wallpaper(value);
        }

        if property == "floor" {
            // The Java `NumberFormatException` on a malformed value.
            let Ok(value) = item.get_custom_data().parse::<i32>() else {
                return Ok(());
            };
            room.get_data_mut().set_floor(value);
        }

        if property == "landscape" {
            room.get_data_mut().set_landscape(item.get_custom_data());
        }

        item.delete();
        RoomDao::save_decorations(&room);

        room.send(&FLATPROPERTY::new(property, item.get_custom_data()));
        inventory.remove_item(&item);

        Ok(())
    }
}
