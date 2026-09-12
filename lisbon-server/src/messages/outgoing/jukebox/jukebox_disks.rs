//! Mirrors `net.h4bbo.lisbon.messages.outgoing.jukebox.JUKEBOX_DISCS`.
use crate::game::player::player_manager::PlayerManager;
use crate::game::song::jukebox::burned_disk::BurnedDisk;
use crate::game::song::song::Song;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct JUKEBOX_DISCS {
    disks: Vec<(BurnedDisk, Song)>,
}

impl JUKEBOX_DISCS {
    /// Mirrors the `JUKEBOX_DISCS(Map<BurnedDisk, Song>)` constructor.
    pub fn new(disks: Vec<(BurnedDisk, Song)>) -> Self {
        Self { disks }
    }
}

impl MessageComposer for JUKEBOX_DISCS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(10);
        response.write_int(self.disks.len() as i32);

        for (burned_disk, song) in &self.disks {
            response.write_int(burned_disk.get_slot_id());
            response.write_int(song.get_id());
            response.write_int(song.get_length());

            response.write_string(song.get_title());
            response.write_string(
                PlayerManager::get_instance()
                    .get_player_data_by_id(song.get_user_id())
                    .map(|player_data| player_data.get_name().to_string())
                    .unwrap_or_default(),
            );
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        334
    }
}
