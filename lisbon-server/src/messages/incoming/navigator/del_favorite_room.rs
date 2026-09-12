//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.DEL_FAVORITE_ROOM`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::room_favourites_dao::RoomFavouritesDao;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DEL_FAVORITE_ROOM;

impl MessageEvent for DEL_FAVORITE_ROOM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_type = reader.read_int();
        let mut room_id = reader.read_int();

        if room_type == 1 {
            room_id -= RoomManager::PUBLIC_ROOM_OFFSET;
        }

        RoomFavouritesDao::remove_favourite_room(player.get_details().get_id(), room_id);

        Ok(())
    }
}
