//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.moderation.REMOVERIGHTS`.
use crate::dao::mysql::room_rights_dao::RoomRightsDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REMOVERIGHTS;

impl MessageEvent for REMOVERIGHTS {
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

        let name = reader.contents().unwrap_or_default();
        let Some(target) = PlayerManager::get_instance().get_player_by_name(&name) else {
            return Ok(());
        };

        let target = target.lock();
        let user_id = target.get_details().get_id();

        if !room.get_rights().contains(&user_id) {
            return Ok(());
        }

        room.remove_right(user_id);
        room.refresh_rights(&target);

        RoomRightsDao::remove_rights(target.get_details(), room.get_data());

        Ok(())
    }
}
