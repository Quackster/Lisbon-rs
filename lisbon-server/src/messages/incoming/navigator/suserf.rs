//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.SUSERF`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::flat_results::FLAT_RESULTS;
use crate::messages::outgoing::navigator::noflatsforuser::NOFLATSFORUSER;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SUSERF;

impl MessageEvent for SUSERF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let room_manager = RoomManager::get_instance();
        let mut room_list = room_manager
            .replace_query_rooms(RoomDao::get_rooms_by_user_id(player.get_details().get_id()));

        if !room_list.is_empty() {
            room_manager.sort_rooms(&mut room_list);
            room_manager.rating_santiy_check(&room_list);

            player.send(&FLAT_RESULTS::new(room_list));
        } else {
            player.send(&NOFLATSFORUSER::new(player.get_details().get_name()));
        }

        Ok(())
    }
}
