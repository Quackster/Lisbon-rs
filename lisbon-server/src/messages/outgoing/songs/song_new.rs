//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SONG_NEW`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SONG_NEW {
    item_id: i32,
    title: String,
}

impl SONG_NEW {
    /// Mirrors the `SONG_NEW(int, String)` constructor.
    pub fn new(id: i32, title: &str) -> Self {
        Self {
            item_id: id,
            title: title.to_string(),
        }
    }
}

impl MessageComposer for SONG_NEW {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.item_id);
        response.write_string(self.title.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        331 // "EK"
    }
}
