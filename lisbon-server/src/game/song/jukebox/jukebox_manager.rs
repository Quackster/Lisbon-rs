//! Mirrors `net.h4bbo.lisbon.game.song.jukebox.JukeboxManager`.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::dao::mysql::jukebox_dao::JukeboxDao;
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::song::jukebox::burned_disk::BurnedDisk;
use crate::game::song::song::Song;

pub struct JukeboxManager;

impl JukeboxManager {
    /// Mirrors `getDisks(long)`.
    pub fn get_disks(&self, soundmachine_id: i64) -> HashMap<BurnedDisk, Song> {
        let mut jukebox_disks: HashMap<BurnedDisk, Song> = HashMap::new();

        for burned_disk in JukeboxDao::get_disks(soundmachine_id) {
            if let Some(song) = SongMachineDao::get_song(burned_disk.get_song_id()) {
                jukebox_disks.insert(burned_disk, song);
            }
        }

        jukebox_disks
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static JukeboxManager {
        static INSTANCE: OnceLock<JukeboxManager> = OnceLock::new();
        INSTANCE.get_or_init(|| JukeboxManager)
    }
}
