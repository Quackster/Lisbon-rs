//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.DELETEFLAT`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::log::Log;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DELETEFLAT;

impl MessageEvent for DELETEFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_id = reader
            .contents()
            .and_then(|contents| contents.parse::<i32>().ok())
            .unwrap_or(0);

        Self::delete(room_id, player.get_details().get_id());

        Ok(())
    }
}

impl DELETEFLAT {
    /// Mirrors the static `delete(int, int)`.
    fn delete(room_id: i32, user_id: i32) {
        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return;
        };
        let room = room_arc.lock();

        if !room.is_owner(user_id) {
            return;
        }

        for item in room.get_items() {
            item.delete();
        }

        for entity in room.get_entities() {
            room.get_entity_manager().leave_room(&room, &*entity, true);
        }

        if !room.try_dispose() {
            Log::get_error_logger().error(format!(
                "Room {} did not want to get disposed by player id {}",
                room_id, user_id
            ));
        }

        RoomDao::delete(&room);
    }
}
