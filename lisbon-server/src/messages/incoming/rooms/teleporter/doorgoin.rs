//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.teleporter.DOORGOIN`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::items::broadcast_teleporter::BROADCAST_TELEPORTER;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DOORGOIN;

impl MessageEvent for DOORGOIN {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let item_id: i32 = reader
            .contents()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if room_user.get_authenticate_teleporter_id() == item_id {
            let Some(item) = room.get_item_manager().get_by_id(item_id) else {
                return Ok(());
            };

            room.send(&BROADCAST_TELEPORTER::new(
                item.as_ref().clone(),
                player.get_details().get_name(),
                false,
            ));
        }

        Ok(())
    }
}
