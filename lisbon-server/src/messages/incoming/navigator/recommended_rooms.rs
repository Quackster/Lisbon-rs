//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.RECOMMENDED_ROOMS`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::recommended_room_list::RECOMMENDED_ROOM_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct RECOMMENDED_ROOMS;

impl MessageEvent for RECOMMENDED_ROOMS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let room_limit = 3;
        let room_manager = RoomManager::get_instance();

        let mut room_list = room_manager
            .replace_query_rooms(RoomDao::get_recommended_rooms(room_limit, 0));

        room_manager.sort_rooms(&mut room_list);
        room_manager.rating_santiy_check(&room_list);

        if room_list.len() < room_limit as usize {
            for room in room_manager
                .replace_query_rooms(RoomDao::get_highest_rated_rooms(room_limit, 0))
            {
                if room_list.len() == room_limit as usize {
                    break;
                }

                room_list.push(room);
            }
        }

        player.send(&RECOMMENDED_ROOM_LIST::new(player, room_list));

        Ok(())
    }
}
