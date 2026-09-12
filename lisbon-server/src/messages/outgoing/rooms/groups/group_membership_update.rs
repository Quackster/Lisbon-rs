//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.groups.GROUP_MEMBERSHIP_UPDATE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct GROUP_MEMBERSHIP_UPDATE {
    instance_id: i32,
    group_id: i32,
    rank_id: i32,
}

impl GROUP_MEMBERSHIP_UPDATE {
    /// Mirrors the `GROUP_MEMBERSHIP_UPDATE(int, int, int)` constructor.
    pub fn new(instance_id: i32, group_id: i32, rank_id: i32) -> Self {
        Self {
            instance_id,
            group_id,
            rank_id,
        }
    }
}

impl MessageComposer for GROUP_MEMBERSHIP_UPDATE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);
        response.write_int(self.group_id);
        response.write_int(self.rank_id);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        310
    }
}
