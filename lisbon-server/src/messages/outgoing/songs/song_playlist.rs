//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SONG_PLAYLIST`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::song::song_playlist::SongPlaylist;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SONG_PLAYLIST {
    song_playlist: Vec<SongPlaylist>,
}

impl SONG_PLAYLIST {
    /// Mirrors the `SONG_PLAYLIST(List<SongPlaylist>)` constructor.
    pub fn new(song_playlist: Vec<SongPlaylist>) -> Self {
        Self { song_playlist }
    }
}

impl MessageComposer for SONG_PLAYLIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
        response.write_int(self.song_playlist.len() as i32);

        for playlist in &self.song_playlist {
            let song = playlist.get_song();

            response.write_int(song.get_id());
            response.write_int(song.get_length());
            response.write_string(song.get_title());
            response.write_string(PlayerDao::get_name(song.get_user_id()).unwrap_or_default());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        323 // "EC"
    }
}
