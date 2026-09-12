//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.settings.FLATINFO`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

// `SETFLATINFO.MAX_ALLOWED_VISITORS` (the incoming class is not ported).
const MAX_ALLOWED_VISITORS: i32 = 50;

#[allow(non_camel_case_types)]
pub struct FLATINFO<'a> {
    player: &'a Player,
    room: &'a Room,
}

impl<'a> FLATINFO<'a> {
    /// Mirrors the `FLATINFO(Player, Room)` constructor.
    pub fn new(player: &'a Player, room: &'a Room) -> Self {
        Self { player, room }
    }
}

impl MessageComposer for FLATINFO<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.room.get_data().allow_super_users());
        response.write_int(self.room.get_data().get_access_type_id());
        response.write_int(self.room.get_id());

        if self.room.is_owner(self.player.get_details().get_id())
            || self.room.get_data().show_owner_name()
            || self.player.has_fuse(&Fuseright::SeeAllRoomowners)
        {
            response.write_string(self.room.get_data().get_owner_name());
        } else {
            response.write_string("-");
        }

        // Is called "marker" in Lingo code.
        response.write_string(self.room.get_model().map(|model| model.get_name()).unwrap_or(""));
        response.write_string(self.room.get_data().get_name());
        response.write_string(self.room.get_data().get_description());
        response.write_bool(self.room.get_data().show_owner_name());

        let category = self.room.get_category();
        response.write_bool(category.as_ref().map_or(false, |category| category.has_allow_trading()));
        response.write_bool(category.is_none());
        response.write_int(self.room.get_data().get_visitors_max());
        response.write_int(MAX_ALLOWED_VISITORS);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        54 // "@v"
    }
}
