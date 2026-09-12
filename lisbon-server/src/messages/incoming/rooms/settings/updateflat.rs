//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.UPDATEFLAT`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct UPDATEFLAT;

impl MessageEvent for UPDATEFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split('/').collect();

        let room_id = data
            .first()
            .and_then(|segment| segment.parse::<i32>().ok())
            .unwrap_or(0);

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let mut room = room_arc.lock();

        if !room.is_owner(player.get_details().get_id()) {
            return Ok(());
        }

        let room_name = StringUtil::filter_input(data.get(1).unwrap_or(&""), true);
        let access_type = StringUtil::filter_input(data.get(2).unwrap_or(&""), true);
        let show_owner = data.get(3).and_then(|segment| segment.parse::<i32>().ok()).unwrap_or(0) == 1;

        let access_type_id = if access_type == "closed" {
            1
        } else if access_type == "password" {
            2
        } else {
            0
        };

        room.get_data_mut().set_name(&room_name);
        room.get_data_mut().set_access_type(access_type_id);
        room.get_data_mut().set_show_owner_name(show_owner);
        RoomDao::save(&room);

        Ok(())
    }
}
