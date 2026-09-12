//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.SpaceCafeTrigger`.
use std::any::Any;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;
use crate::game::room::tasks::space_cafe_task::SpaceCafeTask;


pub struct SpaceCafeTrigger;

impl Trigger for SpaceCafeTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        room: &Room,
        first_entry: bool,
        _custom_args: &[Box<dyn Any>],
    ) {
        if !first_entry {
            return;
        }

        if entity.get_type() != EntityType::Player {
            return;
        }

        room.get_task_manager()
            .schedule_task("SpaceCafeTask", Arc::new(SpaceCafeTask::new(room)), 0, 500);
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(&self, _entity: &dyn Entity, _room: &Room, _custom_args: &[Box<dyn Any>]) {}
}

