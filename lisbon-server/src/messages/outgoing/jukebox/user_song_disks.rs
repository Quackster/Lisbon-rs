//! Mirrors `net.h4bbo.lisbon.messages.outgoing.jukebox.USER_SONG_DISKS`.
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_SONG_DISKS {
    user_disks: Vec<(Item, i32)>,
}

impl USER_SONG_DISKS {
    /// Mirrors the `USER_SONG_DISKS(Map<Item, Integer>)` constructor.
    pub fn new(user_disks: Vec<(Item, i32)>) -> Self {
        Self { user_disks }
    }
}

impl MessageComposer for USER_SONG_DISKS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.user_disks.len() as i32);

        for (item, slot) in &self.user_disks {
            response.write_int(*slot);
            response.write_string(
                item.get_custom_data()
                    .split('\n')
                    .nth(5)
                    .unwrap_or(""),
            );
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        333
    }
}
