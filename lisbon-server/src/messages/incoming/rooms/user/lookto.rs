//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.LOOKTO`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::pathfinder::position::Position;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct LOOKTO;

impl MessageEvent for LOOKTO {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if let Some(current_item) = room_user.get_current_item() {
            match current_item.get_definition().get_interaction_type() {
                Some(InteractionType::WsJoinQueue)
                | Some(InteractionType::WsQueueTile)
                | Some(InteractionType::WsTileStart) => {
                    return Ok(());
                }
                _ => {}
            }
        }

        let contents = reader.contents().unwrap_or_default();
        let data: Vec<&str> = contents.split(" ").collect();

        let look_direction = Position::new_xy(
            data.first().and_then(|value| value.parse().ok()).unwrap_or(0),
            data.get(1).and_then(|value| value.parse().ok()).unwrap_or(0),
        );

        if room_user.contains_status(StatusType::Lay) {
            return Ok(());
        }

        let position = room_user.get_position();

        if position.get_x() == look_direction.get_x()
            && position.get_y() == look_direction.get_y()
            && position.get_z() == look_direction.get_z()
        {
            return Ok(());
        }

        let on_door = room.get_model().is_some_and(|model| {
            let door_location = model.get_door_location();
            door_location.get_x() == position.get_x()
                && door_location.get_y() == position.get_y()
                && door_location.get_z() == position.get_z()
        });

        if on_door {
            return Ok(());
        }

        if let Some(current_item) = room_user.get_current_item() {
            if current_item.has_behaviour(ItemBehaviour::Teleporter) {
                return Ok(());
            }
        }

        let mut position = room_user.get_position();
        let rotation = Rotation::calculate_human_direction(
            position.get_x(),
            position.get_y(),
            look_direction.get_x(),
            look_direction.get_y(),
        );

        // When sitting calculate even rotation
        if room_user.contains_status(StatusType::Sit) {
            let current = position.copy();

            // If they're sitting on the ground also rotate body.
            if room_user.is_sitting_on_ground() {
                position.set_body_rotation(rotation / 2 * 2);
            }

            // And now rotate their head for all sitting people.
            position.set_head_rotation(Rotation::get_head_rotation(
                current.get_rotation(),
                &current,
                &look_direction,
            ));
        } else {
            position.set_rotation(rotation);
        }

        room_user.set_position(position);
        room_user.set_needs_update(true);
        room_user.begin_look_timer();
        room_user.reset_room_timer();

        Ok(())
    }
}
