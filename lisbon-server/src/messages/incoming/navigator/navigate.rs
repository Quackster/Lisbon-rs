//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.NAVIGATE`.
use crate::dao::mysql::navigator_dao::NavigatorDao;
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::navigator::navigator_manager::NavigatorManager;
use crate::game::player::player::Player;
use crate::game::player::player_rank::PlayerRank;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::navigator::navnodeinfo::NAVNODEINFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct NAVIGATE;

impl MessageEvent for NAVIGATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let hide_full = reader.read_int() == 1;
        let mut category_id = reader.read_int();

        if category_id >= RoomManager::PUBLIC_ROOM_OFFSET {
            // A public-room follow: re-resolve the category from the room.
            if let Some(room) = RoomManager::get_instance()
                .get_room_by_id(category_id - RoomManager::PUBLIC_ROOM_OFFSET)
            {
                category_id = room
                    .lock()
                    .get_category()
                    .map(|category| category.get_id())
                    .unwrap_or(category_id);
            }
        }

        let Some(category) = NavigatorManager::get_instance().get_category_by_id(category_id)
        else {
            return Ok(());
        };

        let rank = player
            .get_details()
            .get_rank()
            .unwrap_or(PlayerRank::Normal)
            .rank_id();

        if category.get_minimum_role_access().rank_id() > rank {
            return Ok(());
        }

        let mut sub_categories =
            NavigatorManager::get_instance().get_categories_by_parent_id(category.get_id());
        sub_categories.sort_by(|a, b| b.get_current_visitors().cmp(&a.get_current_visitors()));

        let mut rooms: Vec<crate::game::room::room::Room> = Vec::new();

        let category_current_visitors = category.get_current_visitors();
        let category_max_visitors = category.get_max_visitors();
        let room_manager = RoomManager::get_instance();

        if category.is_public_spaces() {
            for room in room_manager.replace_query_rooms(RoomDao::get_rooms_by_user_id(0)) {
                if room.get_data().is_navigator_hide() {
                    continue;
                }

                if room.get_data().get_category_id() != category.get_id() {
                    continue;
                }

                if hide_full && room.get_data().get_visitors_now() >= room.get_data().get_visitors_max() {
                    continue;
                }

                rooms.push(room);
            }
        } else {
            for room in room_manager
                .replace_query_rooms(NavigatorDao::get_recent_rooms(30, category.get_id()))
            {
                if room.get_data().get_category_id() != category.get_id() {
                    continue;
                }

                if hide_full && room.get_data().get_visitors_now() >= room.get_data().get_visitors_max() {
                    continue;
                }

                rooms.push(room);
            }
        }

        room_manager.sort_rooms(&mut rooms);
        room_manager.rating_santiy_check(&rooms);

        player.send(&NAVNODEINFO::new(
            player,
            category,
            rooms,
            hide_full,
            sub_categories,
            category_current_visitors,
            category_max_visitors,
            rank,
        ));

        Ok(())
    }
}
