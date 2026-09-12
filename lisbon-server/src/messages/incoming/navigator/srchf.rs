//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.SRCHF`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::flat_noresults::FLAT_NORESULTS;
use crate::messages::outgoing::navigator::noflats::NOFLATS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SRCHF;

impl MessageEvent for SRCHF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let search_query = reader.contents().unwrap_or_default();
        let room_manager = RoomManager::get_instance();

        let mut room_list = room_manager
            .replace_query_rooms(RoomDao::query_search_rooms(&search_query));

        if !room_list.is_empty() {
            room_manager.sort_rooms(&mut room_list);
            room_manager.rating_santiy_check(&room_list);

            player.send(&FLAT_NORESULTS::new(room_list, player));
        } else {
            player.send(&NOFLATS);
        }

        Ok(())
    }
}
