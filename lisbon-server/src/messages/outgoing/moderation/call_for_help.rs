//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.CALL_FOR_HELP`.
use crate::game::moderation::cfh::call_for_help::CallForHelp;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CALL_FOR_HELP {
    cfh: CallForHelp,
}

impl CALL_FOR_HELP {
    /// Mirrors the `CALL_FOR_HELP(CallForHelp)` constructor.
    pub fn new(cfh: &CallForHelp) -> Self {
        Self { cfh: cfh.clone() }
    }
}

impl MessageComposer for CALL_FOR_HELP {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let room = self.cfh.get_room();
        let data = room.get_data();

        response.write_string(self.cfh.get_cry_id());
        response.write_int(self.cfh.get_category());
        response.write_string(self.cfh.get_formatted_request_time());
        response.write_string(self.cfh.get_caller());
        response.write_string(self.cfh.get_message());
        response.write_string(self.cfh.get_caller());
        response.write_string(data.get_name());

        if room.is_public_room() {
            response.write_int(0);
            response.write_string(data.get_ccts());
            response.write_int(room.get_id() + RoomManager::PUBLIC_ROOM_OFFSET);
            response.write_int(room.get_id());
        } else {
            response.write_int(1);
            response.write_string(data.get_name());
            response.write_int(room.get_id());
            response.write_string(data.get_owner_name());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        148 // "BT"
    }
}
