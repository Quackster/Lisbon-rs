//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.groups.GROUP_BADGES`.
use std::collections::HashMap;

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GROUP_BADGES {
    group_badges: HashMap<i32, String>,
}

impl GROUP_BADGES {
    /// Mirrors the `GROUP_BADGES(HashMap<Integer, String>)` constructor.
    pub fn new(group_badges: HashMap<i32, String>) -> Self {
        Self { group_badges }
    }
}

impl MessageComposer for GROUP_BADGES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.group_badges.len() as i32);

        for (group_id, badge) in &self.group_badges {
            response.write_int(*group_id);
            response.write_string(badge.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        309
    }
}
