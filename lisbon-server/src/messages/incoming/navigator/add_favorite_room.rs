//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.ADD_FAVORITE_ROOM`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::room_favourites_dao::RoomFavouritesDao;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct ADD_FAVORITE_ROOM;

impl MessageEvent for ADD_FAVORITE_ROOM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_type = reader.read_int();
        let mut room_id = reader.read_int();

        if room_type == 1 {
            room_id -= RoomManager::PUBLIC_ROOM_OFFSET;
        }

        if RoomManager::get_instance().get_room_by_id(room_id).is_none() {
            return Ok(()); // Room was null, ignore request
        }

        // The Java guard iterates the favourite rooms; the Rust
        // `RoomFavouritesDao.getFavouriteRooms` returns the room ids directly.
        for favourite_id in RoomFavouritesDao::get_favourite_rooms(player.get_details().get_id()) {
            if favourite_id == room_id {
                return Ok(()); // Room already added, ignore request
            }
        }

        RoomFavouritesDao::add_favourite_room(player.get_details().get_id(), room_id);

        Ok(())
    }
}
