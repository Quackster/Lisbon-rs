//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.MOVESTUFF`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::items::move_flooritem::MOVE_FLOORITEM;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MOVESTUFF;

impl MessageEvent for MOVESTUFF {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split(' ').collect();

        // The Java `NumberFormatException` on a malformed id.
        let Some(item_id) = data.first().and_then(|value| value.parse::<i32>().ok()) else {
            return Ok(());
        };

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if item.has_behaviour(ItemBehaviour::WallItem) {
            return Ok(());
        }

        // The Java `NumberFormatException` on a malformed coordinate.
        let Some(x) = data.get(1).and_then(|value| value.parse::<f64>().ok()) else {
            return Ok(());
        };
        let Some(y) = data.get(2).and_then(|value| value.parse::<f64>().ok()) else {
            return Ok(());
        };
        let Some(rotation) = data.get(3).and_then(|value| value.parse::<f64>().ok()) else {
            return Ok(());
        };

        let (x, y, rotation) = (x as i32, y as i32, rotation as i32);

        let old_position = item.get_position().copy();

        let is_rotation = item.get_position() == &Position::new_xy(x, y)
            && item.get_position().get_rotation() != rotation;

        if is_rotation {
            if item.get_rolling_data().is_some() {
                // Don't allow rotating when rolling.
                return Ok(());
            }
        }

        if (old_position.get_x() == x
            && old_position.get_y() == y
            && old_position.get_rotation() == rotation)
            || !item.is_valid_move(&item, &room, Some(player), x, y, rotation)
        {
            // Send item update even though we cancelled, otherwise the client
            // will be confused.
            player.send(&MOVE_FLOORITEM::new(item.clone()));
            return Ok(());
        }

        let z = item.get_position().get_z();
        room.get_mapping().lock().move_item(
            &room,
            player,
            &mut item,
            &Position::with_rotations(x, y, z, rotation, rotation),
            &old_position,
        );

        Ok(())
    }
}
