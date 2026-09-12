//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.GETSPACENODEUSERS`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::nodespaceusers::NODESPACEUSERS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETSPACENODEUSERS;

impl MessageEvent for GETSPACENODEUSERS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_handle) = RoomManager::get_instance()
            .get_room_by_id(reader.read_int() - RoomManager::PUBLIC_ROOM_OFFSET)
        else {
            return Ok(());
        };

        let room = room_handle.lock();

        // The `Arc` handles and lock guards are kept alive for the
        // `&Player` references the `NODESPACEUSERS` message borrows.
        let mut player_handles: Vec<Arc<Mutex<Player>>> =
            room.get_entity_manager().get_players();

        let child_rooms: Vec<Arc<Mutex<Room>>> = if room.is_public_room() {
            RoomManager::get_instance().get_child_rooms(&*room)
        } else {
            Vec::new()
        };

        for child_room in child_rooms {
            let child = child_room.lock();
            player_handles.extend(child.get_entity_manager().get_players());
        }

        let mut player_guards: Vec<parking_lot::MutexGuard<'_, Player>> = Vec::new();
        for handle in &player_handles {
            player_guards.push(handle.lock());
        }

        let players: Vec<&Player> = player_guards.iter().map(|guard| &**guard).collect();

        player.send(&NODESPACEUSERS::new(players));

        Ok(())
    }
}
