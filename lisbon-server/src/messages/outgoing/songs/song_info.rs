//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SONG_INFO`.
use crate::game::song::song::Song;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SONG_INFO {
    song: Song,
}

impl SONG_INFO {
    /// Mirrors the `SONG_INFO(Song)` constructor.
    pub fn new(song: Song) -> Self {
        Self { song }
    }
}

impl MessageComposer for SONG_INFO {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.song.get_data());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        300 // "Dl"
    }
}
