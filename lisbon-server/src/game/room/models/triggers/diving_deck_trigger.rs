//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.DivingDeckTrigger`.
use std::any::Any;
use std::sync::Arc;

use parking_lot::Mutex;
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::managers::room_task_manager::Tickable;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;


pub struct DivingDeckTrigger;

impl Trigger for DivingDeckTrigger {
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

        if let Some(room_user) = player.get_room_user() {
            if room_user.get_position().get_z() == 1.0 {
                room_user.set_status(StatusType::Swim, "");
                room_user.set_needs_update(true);
            }
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

/// Mirrors `DivingDeckTrigger.PoolCamera` (the Java `Runnable` is the
/// `run` method; the fields are behind a `Mutex` because the scheduler
/// thread and the trigger share the same task instance).
pub struct PoolCamera {
    room: Room,
    player: Mutex<Option<Arc<Mutex<Player>>>>,
    camera_type: Mutex<i32>,
}

impl PoolCamera {
    /// Mirrors the `PoolCamera(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            room: room.clone(),
            player: Mutex::new(None),
            camera_type: Mutex::new(0),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        if self.player.lock().is_none() {
            self.spectate_new_player();
            self.new_camera_mode(-1);
            return;
        }

        let camera_type = rand::thread_rng().gen_range(0..3);

        match camera_type {
            0 => self.spectate_new_player(),
            1 => self.new_camera_mode(1),
            2 => self.new_camera_mode(2),
            _ => unreachable!(),
        }
    }

    /// Mirrors `spectateNewPlayer()` (the Java recursion is a loop here).
    pub fn spectate_new_player(&self) {
        let player_list = self.room.get_entity_manager().get_players();

        let new_player = if player_list.len() > 1 {
            let mut found = player_list[0].clone();

            loop {
                let candidate = player_list[rand::thread_rng().gen_range(0..player_list.len())].clone();

                let same_player = self
                    .player
                    .lock()
                    .as_ref()
                    .map(|current| {
                        current
                            .lock()
                            .get_details()
                            .get_id()
                            == candidate.lock().get_details().get_id()
                    })
                    .unwrap_or(false);

                if !same_player {
                    found = candidate;
                    break;
                }
            }

            found
        } else {
            player_list[0].clone()
        };

        *self.player.lock() = Some(new_player.clone());

        let instance_id = new_player
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);

        self.room.send(&SHOWPROGRAM::new(vec![
            "cam1".to_string(),
            "targetcamera".to_string(),
            instance_id.to_string(),
        ]));
    }

    /// Mirrors `newCameraMode(int)`.
    pub fn new_camera_mode(&self, mode: i32) {
        let camera_type = if mode > 0 {
            mode
        } else {
            rand::thread_rng().gen_range(1..3)
        };
        *self.camera_type.lock() = camera_type;
        self.room.send(&SHOWPROGRAM::new(vec![
            "cam1".to_string(),
            "setcamera".to_string(),
            camera_type.to_string(),
        ]));
    }

    /// Mirrors `getPlayer()`.
    pub fn get_player(&self) -> Option<Arc<Mutex<Player>>> {
        self.player.lock().clone()
    }

    /// Mirrors `getCameraType()`.
    pub fn get_camera_type(&self) -> i32 {
        *self.camera_type.lock()
    }
}

impl Tickable for PoolCamera {
    fn tick(&self) {
        self.run();
    }
}
