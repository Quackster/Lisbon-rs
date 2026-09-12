//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.settings.SETFLATCAT`.
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::entity::entity::Entity;
use crate::game::navigator::navigator_manager::NavigatorManager;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SETFLATCAT;

impl MessageEvent for SETFLATCAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let room_id = reader.read_int();
        let category_id = reader.read_int();

        let Some(mut category) = NavigatorManager::get_instance().get_category_by_id(category_id) else {
            return Ok(());
        };

        if category.get_minimum_role_set_flat().rank_id()
            > player.get_details().get_rank().map(|rank| rank.rank_id()).unwrap_or(0)
        {
            return Ok(());
        }

        if category.is_node() || category.is_public_spaces() {
            let Some(no_category) = NavigatorManager::get_instance().get_category_by_id(2) else {
                return Ok(());
            };
            category = no_category;
        }

        let Some(room_arc) = RoomManager::get_instance().get_room_by_id(room_id) else {
            return Ok(());
        };
        let mut room = room_arc.lock();

        if !room.is_owner(player.get_details().get_id()) {
            return Ok(());
        }

        room.get_data_mut().set_category_id(category.get_id());
        RoomDao::save(&room);

        Ok(())
    }
}
