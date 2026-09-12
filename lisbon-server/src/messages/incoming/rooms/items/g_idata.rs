//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.G_IDATA`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::items::idata::IDATA;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_IDATA;

impl MessageEvent for G_IDATA {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        let contents = reader.contents().unwrap_or_default();

        if contents.is_empty() || !contents.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let item_id = contents.parse::<i32>().unwrap_or(0);

        let Some(item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if item.has_behaviour(ItemBehaviour::PostIt) {
            let custom_data = item.get_custom_data().to_string();
            let colour = custom_data.get(..6).unwrap_or("");
            let text = if custom_data.len() > 6 {
                custom_data.get(6..).unwrap_or("")
            } else {
                ""
            };

            player.send(&IDATA::new(*item, colour, text));
        } else {
            player.send(&IDATA::item_only(*item));
        }

        Ok(())
    }
}
