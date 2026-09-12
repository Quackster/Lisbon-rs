//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.GETFLATINFO`.
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::recommended_room_list::RECOMMENDED_ROOM_LIST;
use crate::messages::outgoing::rooms::settings::flatinfo::FLATINFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETFLATINFO;

impl MessageEvent for GETFLATINFO {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_id = reader
            .contents()
            .and_then(|contents| contents.parse::<i32>().ok())
            .unwrap_or(0);

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let room = room_arc.lock();

        player.send(&RECOMMENDED_ROOM_LIST::new(player, Vec::new()));
        player.send(&FLATINFO::new(player, &*room));

        Ok(())
    }
}
