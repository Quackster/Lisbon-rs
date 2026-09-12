//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.RooftopRumbleTrigger`.
use std::any::Any;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::room::models::triggers::diving_deck_trigger::PoolCamera;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;

/// Mirrors `RooftopRumbleTrigger` (Java `extends GenericTrigger`; Rust
/// has no subtyping, so the base trigger is composed).
pub struct RooftopRumbleTrigger;

impl Trigger for RooftopRumbleTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        room: &Room,
        _first_entry: bool,
        _custom_args: &[Box<dyn Any>],
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        if room.get_task_manager().has_task("DivingCamera") {
            if let Some(camera) = room.get_task_manager().get_task_as::<PoolCamera>("DivingCamera") {
                let instance_id = camera
                    .get_player()
                    .and_then(|p| p.lock().get_room_user().map(|room_user| room_user.get_instance_id()))
                    .unwrap_or(0);

                player.send(&SHOWPROGRAM::new(vec![
                    "cam1".to_string(),
                    "targetcamera".to_string(),
                    instance_id.to_string(),
                ]));
                player.send(&SHOWPROGRAM::new(vec![
                    "cam1".to_string(),
                    "setcamera".to_string(),
                    camera.get_camera_type().to_string(),
                ]));
            }
        } else {
            room.get_task_manager()
                .schedule_task("DivingCamera", Arc::new(PoolCamera::new(room)), 0, 10_000);
        }
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(&self, entity: &dyn Entity, room: &Room, _custom_args: &[Box<dyn Any>]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        if room.get_entity_manager().get_players().is_empty() {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        if let Some(camera) = room.get_task_manager().get_task_as::<PoolCamera>("DivingCamera") {
            let spectating_this_player = camera
                .get_player()
                .map(|p| p.lock().get_details().get_id() == player.get_details().get_id())
                .unwrap_or(false);

            if spectating_this_player {
                camera.spectate_new_player();
            }
        }
    }
}

