//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.EDIT_SONG`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_info::SONG_INFO;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::outgoing::songs::user_sound_packages::USER_SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct EDIT_SONG;

impl MessageEvent for EDIT_SONG {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
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

        let song_id = reader.read_int();
        let Some(song) = SongMachineDao::get_song(song_id) else {
            return Ok(());
        };

        if song.get_user_id() != player.get_details().get_id() {
            return Ok(());
        }

        player.send(&SONG_INFO::new(song));
        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));

        if let Some(inventory) = player.get_inventory() {
            player.send(&USER_SOUND_PACKAGES::new(inventory.get_soundsets()));
        }

        Ok(())
    }
}
