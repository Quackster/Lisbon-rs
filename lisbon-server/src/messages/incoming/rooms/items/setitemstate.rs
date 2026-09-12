//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.SETITEMSTATE`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SETITEMSTATE;

impl MessageEvent for SETITEMSTATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let item_id_string = reader.read_string();

        if item_id_string.is_empty() || !item_id_string.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let item_id = item_id_string.parse::<i32>().unwrap_or(0);

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if item.get_definition().get_sprite() == "poster" {
            return Ok(());
        }

        if item.has_behaviour(ItemBehaviour::RoomDimmer)
            || item.has_behaviour(ItemBehaviour::Dice)
            || item.has_behaviour(ItemBehaviour::PrizeTrophy)
            || item.has_behaviour(ItemBehaviour::PostIt)
            || item.has_behaviour(ItemBehaviour::WheelOfFortune)
            || item.has_behaviour(ItemBehaviour::Photo)
            || item.has_behaviour(ItemBehaviour::SoundMachineSampleSet)
        {
            return Ok(());
        }

        if item.get_definition().has_behaviour(ItemBehaviour::RequiresRightsForInteraction)
            && !room.has_rights(player.get_details().get_id())
        {
            return Ok(());
        }

        let custom_data = reader.read_int().to_string();
        item.set_custom_data(&custom_data);
        item.update_status();
        item.save();

        Ok(())
    }
}
