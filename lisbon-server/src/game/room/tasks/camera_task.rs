//! Mirrors `net.h4bbo.lisbon.game.room.tasks.CameraTask`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;

/// Mirrors `CameraTask` (the Java `Runnable` is the `run` method).
pub struct CameraTask {
    player: Arc<Mutex<Player>>,
}

impl CameraTask {
    /// Mirrors the `CameraTask(Entity)` constructor (the `USEITEM` caller
    // hands over the player).
    pub fn new(player: Arc<Mutex<Player>>) -> Self {
        Self { player }
    }

    /// Mirrors `run()` (the Java `getStatus(USE_ITEM)` NPE is an empty
    // value here).
    pub fn run(&self) {
        let player = self.player.lock();
        let entity: &(dyn Entity + Send) = &*player;

        let Some(room_user) = entity.get_room_user() else {
            return;
        };

        if room_user.get_room().is_none() {
            return;
        }

        let item = room_user
            .get_status(StatusType::UseItem)
            .map(|status| status.get_value().to_string())
            .unwrap_or_default();

        room_user.remove_status(StatusType::UseItem);
        room_user.set_status(StatusType::CarryItem, &item);

        if !room_user.is_walking() {
            if let Some(room) = room_user.get_room() {
                let refs: Vec<&(dyn Entity + Send)> = vec![entity];
                room.send(&USER_STATUSES::new(refs));
            }
        }
    }
}
