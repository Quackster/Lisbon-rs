//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.ADDSTRIPITEM`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct ADDSTRIPITEM;

impl MessageEvent for ADDSTRIPITEM {
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

        // The Java `NumberFormatException` when `data[2]` is missing or
        // non-numeric.
        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split(' ').collect();
        let Some(item_id) = data.get(2).and_then(|value| value.parse::<i32>().ok()) else {
            return Ok(());
        };

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if item.has_behaviour(ItemBehaviour::PostIt) {
            // The client does not allow picking up post-it's, thus neither
            // will the server
            return Ok(());
        }

        item.set_owner_id(player.get_details().get_id());
        room.get_mapping().lock().pickup_item(&room, player, &mut item);

        if let Some(inventory) = player.get_inventory() {
            inventory.add_item(&item);
            inventory.view(player, "new");
        }

        Ok(())
    }
}
