//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.SETITEMDATA`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SETITEMDATA;

impl MessageEvent for SETITEMDATA {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        let contents = reader.contents().unwrap_or_default();
        let Some(slash_index) = contents.find('/') else {
            return Ok(());
        };

        let item_id = contents[..slash_index].parse::<i32>().unwrap_or(0);
        let item_id_length = item_id.to_string().len();

        let after_id = &contents[item_id_length + 1..];
        let colour = after_id.get(..6).unwrap_or("");
        let mut new_message = StringUtil::filter_input(&contents[item_id_length + 8..], false);

        if colour != "FFFFFF"
            && colour != "FFFF33"
            && colour != "FF9CFF"
            && colour != "9CFF9C"
            && colour != "9CCEFF"
        {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::PostIt) {
            return Ok(());
        }

        let old_text = if item.get_custom_data().len() > 6 {
            item.get_custom_data()[6..].to_string()
        } else {
            String::new()
        };

        // If the user doesn't have rights, they may only append to the sticky.
        if !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            if !new_message.starts_with(&old_text) {
                return Ok(());
            }
        }

        if new_message.len() > 684 {
            new_message.truncate(684);
        }

        item.set_custom_data(&format!("{}{}", colour, new_message));
        item.update_status();
        item.save();

        Ok(())
    }
}
