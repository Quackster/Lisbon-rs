//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.GETFVRF`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::favouriteroomresults::FAVOURITEROOMRESULTS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETFVRF;

impl MessageEvent for GETFVRF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let favourite_rooms =
            RoomManager::get_instance().get_favourite_rooms(player.get_details().get_id());

        let favourite_public_rooms: Vec<_> = favourite_rooms
            .iter()
            .filter(|room| room.is_public_room())
            .cloned()
            .collect();
        let favourite_flat_rooms: Vec<_> = favourite_rooms
            .iter()
            .filter(|room| !room.is_public_room())
            .cloned()
            .collect();

        player.send(&FAVOURITEROOMRESULTS::new(
            player,
            favourite_public_rooms,
            favourite_flat_rooms,
        ));

        Ok(())
    }
}
