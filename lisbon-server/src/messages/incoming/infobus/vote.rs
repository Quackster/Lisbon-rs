//! Mirrors `net.h4bbo.lisbon.messages.incoming.infobus.VOTE`.
use crate::dao::mysql::infobus_dao::InfobusDao;
use crate::game::entity::entity::Entity;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct VOTE;

impl MessageEvent for VOTE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        let Some(model) = room.get_model() else {
            return Ok(());
        };
        if model.get_name() != "park_b" {
            return Ok(());
        }

        let Some(current_poll) = InfobusManager::get_instance().get_current_poll() else {
            return Ok(());
        };

        let choice = reader.read_int();

        if choice <= 0 || choice > current_poll.get_poll_data().get_answers().len() as i32 {
            return Ok(());
        }

        if InfobusDao::has_answer(current_poll.get_id(), player.get_details().get_id()) {
            return Ok(());
        }

        InfobusDao::add_answer(
            current_poll.get_id(),
            choice - 1,
            player.get_details().get_id(),
        );

        if InfobusManager::get_instance().can_update_results() {
            InfobusManager::get_instance().show_poll_results(current_poll.get_id());
        }

        Ok(())
    }
}
