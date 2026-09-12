//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.moderation.REMOVEALLRIGHTS`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::dao::mysql::room_rights_dao::RoomRightsDao;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REMOVEALLRIGHTS;

impl MessageEvent for REMOVEALLRIGHTS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_id = reader.read_int();

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let room = room_arc.lock();

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        room.clear_rights();

        for room_player in room.get_entity_manager().get_players() {
            let guard = room_player.lock();
            room.refresh_rights(&guard);
        }

        RoomRightsDao::delete_room_rights(room.get_data());

        Ok(())
    }
}
