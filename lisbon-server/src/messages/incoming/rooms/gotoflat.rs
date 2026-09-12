//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.GOTOFLAT`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GOTOFLAT;

impl MessageEvent for GOTOFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_id) = reader
            .contents()
            .and_then(|contents| contents.parse::<i32>().ok())
        else {
            return Ok(());
        };

        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if room_user.get_authenticate_id() != room_id {
            return Ok(());
        }

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let room = room_arc.lock();

        room.get_entity_manager().enter_room_entity(&room, player, None);

        Ok(())
    }
}
