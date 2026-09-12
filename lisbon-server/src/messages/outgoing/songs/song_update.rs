//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SONG_UPDATE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct SONG_UPDATE;

impl MessageComposer for SONG_UPDATE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        339 // "ES"
    }
}
