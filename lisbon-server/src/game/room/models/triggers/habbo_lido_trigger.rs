//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.HabboLidoTrigger`.
use std::any::Any;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;


pub struct HabboLidoTrigger;

impl Trigger for HabboLidoTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        _room: &Room,
        _first_entry: bool,
        _custom_args: &[Box<dyn Any>],
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        if let Some(room_user) = player.get_room_user() {
            if room_user.get_position().get_z() == 1.0 {
                room_user.set_status(StatusType::Swim, "");
                room_user.set_needs_update(true);
            }
        }
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(&self, _entity: &dyn Entity, _room: &Room, _custom_args: &[Box<dyn Any>]) {}
}

