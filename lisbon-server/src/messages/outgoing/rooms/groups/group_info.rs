//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.groups.GROUP_INFO`.
use crate::game::groups::group::Group;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GROUP_INFO {
    group: Group,
}

impl GROUP_INFO {
    /// Mirrors the `GROUP_INFO(Group)` constructor.
    pub fn new(group: Group) -> Self {
        Self { group }
    }
}

impl MessageComposer for GROUP_INFO {
    /// Mirrors `compose(NettyResponse)`.
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.group.get_id());
        response.write_string(self.group.get_name());
        response.write_string(self.group.get_description());

        let room = RoomManager::get_instance().get_room_by_id(self.group.get_room_id());

        if self.group.get_room_id() > 0 && room.is_some() {
            response.write_int(self.group.get_room_id());
            response.write_string(room.unwrap().lock().get_data().get_name());
        } else {
            response.write_int(-1);
            response.write_string("");
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        311
    }
}
