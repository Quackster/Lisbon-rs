//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.FAVOURITEROOMRESULTS`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FAVOURITEROOMRESULTS<'a> {
    node_type: i32,
    viewer: &'a Player,
    favourite_public_rooms: Vec<Room>,
    favourite_flat_rooms: Vec<Room>,
}

impl<'a> FAVOURITEROOMRESULTS<'a> {
    /// Mirrors the `FAVOURITEROOMRESULTS(Player, List<Room>, List<Room>)`
    /// constructor.
    pub fn new(
        viewer: &'a Player,
        favourite_public_rooms: Vec<Room>,
        favourite_flat_rooms: Vec<Room>,
    ) -> Self {
        Self {
            node_type: 2,
            viewer,
            favourite_public_rooms,
            favourite_flat_rooms,
        }
    }

    /// Mirrors the Java inline door/description split.
    fn parse_public_description(description: &str) -> (String, i32) {
        if description.contains('/') {
            let data = description.split('/').collect::<Vec<_>>();
            (
                data.first().map(|part| part.to_string()).unwrap_or_default(),
                data.get(1)
                    .and_then(|part| part.parse::<i32>().ok())
                    .unwrap_or(0),
            )
        } else {
            (description.to_string(), 0)
        }
    }
}

impl MessageComposer for FAVOURITEROOMRESULTS<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
        response.write_int(0);
        response.write_int(self.node_type);
        response.write_string("");
        response.write_int(0);
        response.write_int(0);
        response.write_int(0);

        if self.node_type == 2 {
            response.write_int(self.favourite_flat_rooms.len() as i32);

            for room in &self.favourite_flat_rooms {
                response.write_int(room.get_id());
                response.write_string(room.get_data().get_name());

                if room.is_owner(self.viewer.get_details().get_id())
                    || room.get_data().show_owner_name()
                    || self.viewer.has_fuse(&Fuseright::SeeAllRoomowners)
                {
                    response.write_string(room.get_data().get_owner_name());
                } else {
                    response.write_string("-");
                }

                response.write_string(room.get_data().get_access_type());
                response.write_int(room.get_data().get_visitors_now());
                response.write_int(room.get_data().get_visitors_max());
                response.write_string(room.get_data().get_description());
            }
        }

        for room in &self.favourite_public_rooms {
            let (description, door) =
                Self::parse_public_description(room.get_data().get_description());

            response.write_int(room.get_id() + RoomManager::PUBLIC_ROOM_OFFSET);
            response.write_int(1);
            response.write_string(room.get_data().get_name());
            response.write_int(room.get_data().get_total_visitors_now(room));
            response.write_int(room.get_data().get_total_visitors_max(room));
            response.write_int(room.get_data().get_category_id());
            response.write_string(description);
            response.write_int(room.get_id());
            response.write_int(door);
            response.write_string(room.get_data().get_ccts());
            response.write_int(0);
            response.write_int(1);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        61
    }
}
