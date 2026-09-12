//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.teleporter.GETDOORFLAT`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::types::teleport_interactor::TeleportInteractor;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETDOORFLAT;

impl MessageEvent for GETDOORFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let item_id: i32 = reader
            .contents()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Teleporter) {
            return Ok(());
        }

        TeleportInteractor::new().on_interact(player, &room, &mut *item, 2);

        Ok(())
    }
}
