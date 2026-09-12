//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.NAVNODEINFO`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::navigator::navigator_category::NavigatorCategory;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct NAVNODEINFO<'a> {
    viewer: &'a Player,
    parent_category: NavigatorCategory,
    rooms: Vec<Room>,
    hide_full: bool,
    sub_categories: Vec<NavigatorCategory>,
    category_current_visitors: i32,
    category_max_visitors: i32,
    rank: i32,
}

impl<'a> NAVNODEINFO<'a> {
    /// Mirrors the 8-arg `NAVNODEINFO` constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        viewer: &'a Player,
        parent_category: NavigatorCategory,
        rooms: Vec<Room>,
        hide_full: bool,
        sub_categories: Vec<NavigatorCategory>,
        category_current_visitors: i32,
        category_max_visitors: i32,
        rank: i32,
    ) -> Self {
        Self {
            viewer,
            parent_category,
            rooms,
            hide_full,
            sub_categories,
            category_current_visitors,
            category_max_visitors,
            rank,
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

impl MessageComposer for NAVNODEINFO<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.hide_full);
        response.write_int(self.parent_category.get_id());
        response.write_int(if self.parent_category.is_public_spaces() {
            0
        } else {
            2
        });
        response.write_string(self.parent_category.get_name());
        response.write_int(self.category_current_visitors);
        response.write_int(self.category_max_visitors);
        response.write_int(self.parent_category.get_parent_id());

        if !self.parent_category.is_public_spaces() {
            response.write_int(self.rooms.len() as i32);
        }

        for room in &self.rooms {
            if room.is_public_room() {
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
            } else {
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

        for sub_category in &self.sub_categories {
            if sub_category.get_minimum_role_access().rank_id() > self.rank {
                continue;
            }

            response.write_int(sub_category.get_id());
            response.write_int(0);
            response.write_string(sub_category.get_name());
            response.write_int(sub_category.get_current_visitors());
            response.write_int(sub_category.get_max_visitors());
            response.write_int(self.parent_category.get_id());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        220 // "C\""
    }
}
