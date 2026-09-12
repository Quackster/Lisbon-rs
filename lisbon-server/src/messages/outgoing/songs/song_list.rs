//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SONG_LIST`.
use crate::game::song::song::Song;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SONG_LIST {
    song_list: Vec<Song>,
}

impl SONG_LIST {
    /// Mirrors the `SONG_LIST(List<Song>)` constructor.
    pub fn new(song_list: Vec<Song>) -> Self {
        Self { song_list }
    }
}

impl MessageComposer for SONG_LIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.song_list.len() as i32);

        for song in &self.song_list {
            response.write_int(song.get_id());
            response.write_int(song.get_length());
            response.write_string(song.get_title());
            response.write_bool(song.is_burnt());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        322 // "EB"
    }
}
