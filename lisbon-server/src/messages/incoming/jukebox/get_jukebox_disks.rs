//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.GET_JUKEBOX_DISCS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::song::jukebox::jukebox_manager::JukeboxManager;
use crate::messages::outgoing::jukebox::jukebox_disks::JUKEBOX_DISCS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_JUKEBOX_DISCS;

impl MessageEvent for GET_JUKEBOX_DISCS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        // The Java guard is a commented-out no-op (`//return;`), so nothing
        // is evaluated here.

        room.send(&JUKEBOX_DISCS::new(
            JukeboxManager::get_instance()
                .get_disks(sound_machine.get_id() as i64)
                .into_iter()
                .collect(),
        ));

        Ok(())
    }
}
