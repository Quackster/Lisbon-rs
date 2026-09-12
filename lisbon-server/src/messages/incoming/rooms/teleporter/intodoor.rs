//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.teleporter.INTODOOR`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::types::teleport_interactor::TeleportInteractor;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct INTODOOR;

impl MessageEvent for INTODOOR {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let contents = reader.contents().unwrap_or_default();

        if contents.is_empty() || !contents.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let Some(mut item) = room
            .get_item_manager()
            .get_by_id(contents.parse::<i32>().unwrap_or(0))
        else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Teleporter) {
            return Ok(());
        }

        if room_user.get_authenticate_teleporter_id() != -1 {
            return Ok(());
        }

        let item_position = item.get_position();
        let position = room_user.get_position();

        if !item_position.touches(&position)
            && !(
                item_position.get_x() == position.get_x()
                    && item_position.get_y() == position.get_y()
                    && item_position.get_z() == position.get_z()
            )
        {
            return Ok(());
        }

        TeleportInteractor::new().on_interact(player, &room, &mut *item, 1);

        Ok(())
    }
}
