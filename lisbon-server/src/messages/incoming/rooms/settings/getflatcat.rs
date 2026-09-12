//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.GETFLATCAT`.
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::rooms::settings::flatcat::FLATCAT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETFLATCAT;

impl MessageEvent for GETFLATCAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_id = reader.read_int();

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let room = room_arc.lock();

        player.send(&FLATCAT::new(room.get_id(), room.get_data().get_category_id()));

        Ok(())
    }
}
