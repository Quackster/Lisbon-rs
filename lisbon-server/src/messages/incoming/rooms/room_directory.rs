//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.ROOM_DIRECTORY`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::cantconnect::{CANTCONNECT, QueueError};
use crate::messages::outgoing::rooms::open_connection::OPEN_CONNECTION;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct ROOM_DIRECTORY;

impl MessageEvent for ROOM_DIRECTORY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let is_public = reader.contents().and_then(|contents| contents.chars().next());

        if is_public != Some('A') {
            player.send(&OPEN_CONNECTION);
            return Ok(());
        }

        reader.read_bytes(1); // strip 'A'
        let room_id = reader.read_int();

        if room_id == -1 {
            let Some(game_player_arc) = player
                .get_room_user()
                .and_then(|room_user| room_user.get_game_player())
            else {
                return Ok(());
            };

            let game_player = game_player_arc.lock();

            if game_player.is_entering_game() {
                if let Some(game) = game_player.get_game() {
                    let room = game.get_room();
                    room.get_entity_manager().enter_room_entity(
                        &room,
                        player,
                        Some(game_player.get_spawn_position()),
                    );
                }
            }

            return Ok(());
        }

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let room = room_arc.lock();

        if room.is_club_only() && !player.get_details().has_club_subscription() {
            player.send(&CANTCONNECT::new_queue_error(QueueError::ClubOnly));
            return Ok(());
        }

        if room
            .get_data()
            .get_total_visitors_now(&room)
            >= room.get_data().get_total_visitors_max(&room)
            && !player.has_fuse(&Fuseright::EnterFullRooms)
        {
            player.send(&CANTCONNECT::new_queue_error(QueueError::Full));
            return Ok(());
        }

        room.get_entity_manager().enter_room_entity(&room, player, None);

        Ok(())
    }
}
