//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.NEW_SONG`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::outgoing::songs::user_sound_packages::USER_SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct NEW_SONG;

impl MessageEvent for NEW_SONG {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id()) && !player.has_fuse(&Fuseright::AnyRoomController) {
            return Ok(());
        }

        let Some(sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        // We don't want a user to get kicked when making cool beats.
        room_user.reset_room_timer();

        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));

        if let Some(inventory) = player.get_inventory() {
            player.send(&USER_SOUND_PACKAGES::new(inventory.get_soundsets()));
        }

        Ok(())
    }
}
