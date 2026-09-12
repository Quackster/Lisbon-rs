//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.SOUND_PACKAGES`.
use std::collections::HashMap;

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SOUND_PACKAGES {
    tracks: HashMap<i32, i32>,
}

impl SOUND_PACKAGES {
    /// Mirrors the `SOUND_PACKAGES(Map<Integer, Integer>)` constructor.
    pub fn new(tracks: HashMap<i32, i32>) -> Self {
        Self { tracks }
    }
}

impl MessageComposer for SOUND_PACKAGES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(4);
        response.write_int(self.tracks.len() as i32);

        for (slot_id, sound_set) in &self.tracks {
            response.write_int(*slot_id);
            response.write_int(*sound_set);
            response.write_int(9);

            let v = sound_set * 9 - 8;

            for j in v..=v + 8 {
                response.write_int(j);
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        301 // "Dm"
    }
}
