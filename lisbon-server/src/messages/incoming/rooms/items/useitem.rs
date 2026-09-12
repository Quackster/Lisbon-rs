//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.USEITEM`.
use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::tasks::camera_task::CameraTask;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct USEITEM;

impl MessageEvent for USEITEM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if !room_user.contains_status(StatusType::CarryItem) {
            return Ok(());
        }

        let carry_value = room_user
            .get_status(StatusType::CarryItem)
            .map(|status| status.get_value().to_string())
            .unwrap_or_default();

        room_user.remove_status(StatusType::CarryItem);
        room_user.set_status(StatusType::UseItem, &carry_value);

        if !room_user.is_walking() {
            if let Some(room) = room_user.get_room() {
                room.send(&USER_STATUSES::new(vec![player]));
            }
        }

        if let Some(player_arc) =
            PlayerManager::get_instance().get_player_by_id(player.get_details().get_id())
        {
            let task = CameraTask::new(player_arc);
            GameScheduler::get_instance().schedule(
                move || {
                    task.run();
                },
                1000,
            );
        }

        Ok(())
    }
}
