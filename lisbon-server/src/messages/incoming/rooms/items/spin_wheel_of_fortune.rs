//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.SPIN_WHEEL_OF_FORTUNE`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::game_scheduler::GameScheduler;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::game::room::tasks::fortune_task::FortuneTask;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SPIN_WHEEL_OF_FORTUNE;

impl MessageEvent for SPIN_WHEEL_OF_FORTUNE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        let item_id = reader.read_int();

        if item_id < 0 {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::WheelOfFortune) || item.get_requires_update() {
            return Ok(());
        }

        item.set_custom_data("-1");
        item.update_status();
        item.set_requires_update(true);

        let mut task = FortuneTask::new(*item);
        GameScheduler::get_instance().schedule(move || task.run(), 4250);

        Ok(())
    }
}
