//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.JUKEBOX_PLAYLIST_ADD`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::song::jukebox::burned_disk::BurnedDisk;
use crate::game::song::jukebox::jukebox_manager::JukeboxManager;
use crate::game::song::song_playlist::SongPlaylist;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct JUKEBOX_PLAYLIST_ADD;

impl MessageEvent for JUKEBOX_PLAYLIST_ADD {
    /// Mirrors `handle(Player, NettyRequest)`.
    #[allow(unreachable_code)]
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(_sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        // The Java guard is a commented-out no-op (`//return;`), so nothing
        // is evaluated here.

        let song_id = reader.read_int();
        SongMachineDao::remove_playlist_song(song_id, _sound_machine.get_id() as i64);

        let mut playlist: Vec<SongPlaylist> = SongMachineDao::get_song_playlist(_sound_machine.get_id());

        let loaded_discs = JukeboxManager::get_instance().get_disks(_sound_machine.get_id() as i64);

        // Don't load a song if it's not in the jukebox
        if !loaded_discs.keys().any(|disc| disc.get_song_id() == song_id) {
            return Ok(());
        }

        let mut sorted_disks: Vec<&BurnedDisk> = loaded_discs.keys().collect();
        sorted_disks.sort_by_key(|disc| disc.get_slot_id());

        let new_slot_id = sorted_disks
            .first()
            .map(|disc| disc.get_slot_id())
            .unwrap_or(0)
            + 1;

        SongMachineDao::add_playlist(_sound_machine.get_id(), song_id, new_slot_id);

        let Some(song) = SongMachineDao::get_song(song_id) else {
            return Ok(());
        };
        playlist.push(SongPlaylist::new(_sound_machine.get_id(), song, new_slot_id));

        room.send(&SONG_PLAYLIST::new(playlist));

        Ok(())
    }
}
