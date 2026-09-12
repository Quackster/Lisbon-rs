//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.THROW_DICE`.
use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::game::room::tasks::dice_task::DiceTask;
use crate::messages::outgoing::rooms::items::dice_value::DICE_VALUE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct THROW_DICE;

impl MessageEvent for THROW_DICE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let contents = reader.contents().unwrap_or_default();

        if contents.is_empty() || !contents.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let item_id = contents.parse::<i32>().unwrap_or(0);

        if item_id < 0 {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Dice) || item.get_requires_update() {
            return Ok(());
        }

        // Check the user is next to the dice (item tile resolved via the mapping
        // handle, since `Item::get_tile` is a stub).
        let user_touches = match (
            room_user.get_tile(),
            room.get_mapping().lock().get_tile_handle_for(
                &room,
                item.get_position().get_x(),
                item.get_position().get_y(),
            ),
        ) {
            (Some(user_tile), Some(item_tile)) => {
                let user_tile = user_tile.lock();
                let item_tile = item_tile.lock();
                user_tile.get_position().touches(item_tile.get_position())
            }
            _ => false,
        };

        if !user_touches {
            return Ok(());
        }

        room_user.reset_room_timer();

        room.send(&DICE_VALUE::new(item_id, true, 0));

        item.set_custom_data("-1");
        item.update_status();
        item.set_requires_update(true);

        let mut task = DiceTask::new(*item);
        GameScheduler::get_instance().schedule(move || task.run(), 2000);

        Ok(())
    }
}
