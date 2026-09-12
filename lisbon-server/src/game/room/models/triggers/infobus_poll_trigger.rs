//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.InfobusPollTrigger`.
use std::any::Any;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;


pub struct InfobusPollTrigger;

impl Trigger for InfobusPollTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)` (an empty
    /// Java body beyond the `Player` type check).
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
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)` (an empty Java body
    // beyond the `Player` type check).
    fn on_room_leave(
        &self,
        entity: &dyn Entity,
        _room: &Room,
        _custom_args: &[Box<dyn Any>],
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }
    }
}

