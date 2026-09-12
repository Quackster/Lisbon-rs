//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.SETFLATINFO`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SETFLATINFO;

impl SETFLATINFO {
    pub const MAX_ALLOWED_VISITORS: i32 = 50;
}

impl MessageEvent for SETFLATINFO {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let mut contents = reader.contents().unwrap_or_default();

        if let Some(stripped) = contents.strip_prefix('/') {
            contents = stripped.to_string();
        }

        let room_id = contents
            .split('/')
            .next()
            .and_then(|segment| segment.parse::<i32>().ok())
            .unwrap_or(0);

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let mut room = room_arc.lock();

        if !room.is_owner(player.get_details().get_id()) {
            return Ok(());
        }

        for setting in contents.split('\r') {
            let Some(index) = setting.find('=') else {
                continue;
            };

            let key = &setting[..index];
            let value = &setting[index + 1..];

            if key.starts_with("description") {
                room.get_data_mut().set_description(&StringUtil::filter_input(value, true));
            }

            if key.starts_with("allsuperuser") {
                room.get_data_mut().set_super_users(value.parse::<i32>().unwrap_or(0) == 1);
            }

            if key.starts_with("maxvisitors") {
                let mut max_visitors = value.parse::<i32>().unwrap_or(25);

                if max_visitors < 10 || max_visitors > SETFLATINFO::MAX_ALLOWED_VISITORS {
                    max_visitors = 25;
                }

                room.get_data_mut().set_visitors_max(max_visitors);
            }

            if key.starts_with("password") {
                room.get_data_mut().set_password(Some(&StringUtil::filter_input(value, true)));
            }
        }

        RoomDao::save(&room);

        Ok(())
    }
}
