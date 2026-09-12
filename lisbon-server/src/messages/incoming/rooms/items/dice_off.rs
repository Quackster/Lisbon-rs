//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.DICE_OFF`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::items::dice_value::DICE_VALUE;
use crate::messages::outgoing::rooms::items::stuff_data_update::STUFFDATAUPDATE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DICE_OFF;

impl MessageEvent for DICE_OFF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let contents = reader.contents().unwrap_or_default();

        if !contents.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let item_id: i32 = contents.parse().unwrap_or(0);

        if item_id < 0 {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Dice) {
            return Ok(());
        }

        {
            let item_tile = item.get_tile();
            let room_user_tile = room_user.get_tile();

            let item_pos = item_tile.as_ref().map(|arc| arc.lock().get_position().copy());
            let user_pos = room_user_tile.as_ref().map(|arc| arc.lock().get_position().copy());

            match (item_pos, user_pos) {
                (Some(ip), Some(up)) if up.touches(&ip) => {}
                _ => return Ok(()),
            }
        }

        room.send(&DICE_VALUE::new(item_id, false, 0));
        room.send(&STUFFDATAUPDATE::new(item.clone()));

        item.set_custom_data("0");
        item.save();

        Ok(())
    }
}
