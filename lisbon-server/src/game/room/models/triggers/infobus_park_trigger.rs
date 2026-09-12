//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.InfobusParkTrigger`.
use std::any::Any;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;


pub struct InfobusParkTrigger;

impl Trigger for InfobusParkTrigger {
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

        if let Some(player) = entity.as_player() {
            InfobusManager::get_instance().send_door_status(player);
        }
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(&self, _entity: &dyn Entity, _room: &Room, _custom_args: &[Box<dyn Any>]) {}
}

