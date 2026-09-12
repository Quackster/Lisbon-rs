//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.teleporter.GOVIADOOR`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::rooms::open_connection::OPEN_CONNECTION;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GOVIADOOR;

impl MessageEvent for GOVIADOOR {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(_room) = room_user.get_room() else {
            return Ok(());
        };

        if room_user.get_authenticate_teleporter_id() == -1 {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split("/").collect();
        let room_id: i32 = data.first().and_then(|value| value.parse().ok()).unwrap_or(0);
        let item_id: i32 = data.get(1).and_then(|value| value.parse().ok()).unwrap_or(0);

        let linked_teleporter = ItemDao::get_item(item_id);
        let target = RoomManager::get_instance().get_room_by_id(room_id);

        if linked_teleporter.is_some() && target.is_some() {
            room_user.set_authenticate_id(room_id);
            player.send(&OPEN_CONNECTION);
        } else {
            room_user.set_authenticate_teleporter_id(-1);
        }

        Ok(())
    }
}
