//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.TRYFLAT`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::alert::localised_error::LOCALISED_ERROR;
use crate::messages::outgoing::rooms::doorbell_wait::DOORBELL_WAIT;
use crate::messages::outgoing::rooms::flat_letin::FLAT_LETIN;
use crate::messages::outgoing::rooms::flatnotallowedtoenter::FLATNOTALLOWEDTOENTER;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRYFLAT;

impl MessageEvent for TRYFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let contents = reader.contents().unwrap_or_default();

        if contents.is_empty() {
            return Ok(());
        }

        let mut room_id = -1;
        let mut password = String::new();

        if contents.contains('/') {
            let parts: Vec<&str> = contents.split('/').collect();
            let room_id_str = parts.first().copied().unwrap_or("");

            if !room_id_str.is_empty() && room_id_str.chars().all(|c| c.is_ascii_digit()) {
                room_id = room_id_str.parse::<i32>().unwrap_or(-1);
            }

            password = parts.get(1).copied().unwrap_or_default().to_string();
        } else {
            room_id = contents.parse::<i32>().unwrap_or(-1);
        }

        let room = RoomManager::get_instance()
            .get_room_by_id(room_id)
            .map(|arc| arc.lock().clone())
            .or_else(|| RoomDao::get_room_by_id(room_id));

        let Some(room) = room else {
            return Ok(());
        };

        let authenticate_id = player
            .get_room_user()
            .map(|room_user| room_user.get_authenticate_id())
            .unwrap_or(-1);

        if !player.has_fuse(&Fuseright::EnterLockedRooms) && authenticate_id != room_id {
            if room.get_data().get_access_type_id() == 1
                && !room.has_rights_with_super_users(player.get_details().get_id(), false)
                && !player.has_fuse(&Fuseright::AnyRoomController)
            {
                if Self::rang_doorbell(&room, player) {
                    player.send(&DOORBELL_WAIT::new());
                } else {
                    player.send(&FLATNOTALLOWEDTOENTER);
                }

                return Ok(());
            }

            if room.get_data().get_access_type_id() == 2
                && !room.is_owner(player.get_details().get_id())
                && !player.has_fuse(&Fuseright::AnyRoomController)
            {
                if password != room.get_data().get_password().unwrap_or_default() {
                    player.send(&LOCALISED_ERROR::new("Incorrect flat password"));
                    return Ok(());
                }
            }
        }

        player.get_room_user().map(|room_user| room_user.set_authenticate_id(room_id));
        player.send(&FLAT_LETIN);

        Ok(())
    }
}

impl TRYFLAT {
    /// Mirrors the private `rangDoorbell(Room, Player)`.
    fn rang_doorbell(room: &Room, player: &Player) -> bool {
        let mut sent_with_rights = false;

        for user_arc in room.get_entity_manager().get_players() {
            let user = user_arc.lock();

            if !room.has_rights(user.get_details().get_id()) {
                continue;
            }

            user.send(&DOORBELL_WAIT::with_username(player.get_details().get_name()));
            sent_with_rights = true;
        }

        sent_with_rights
    }
}
