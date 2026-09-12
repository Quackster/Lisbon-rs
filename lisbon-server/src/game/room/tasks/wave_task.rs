//! Mirrors `net.h4bbo.lisbon.game.room.tasks.WaveTask`.
use crate::game::entity::entity::Entity;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;

/// Mirrors `WaveTask` (the Java `Runnable` is the `run` method).
pub struct WaveTask {
    entity: Box<dyn Entity + Send>,
}

impl WaveTask {
    /// Mirrors the `WaveTask(Entity)` constructor.
    pub fn new(entity: Box<dyn Entity + Send>) -> Self {
        Self { entity }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let entity = self.entity.as_ref();

        let Some(room_user) = entity.get_room_user() else {
            return;
        };

        if room_user.get_room().is_none() {
            return;
        }

        room_user.remove_status(StatusType::Wave);

        if !room_user.is_walking() {
            if let Some(room) = room_user.get_room() {
                let refs: Vec<&(dyn Entity + Send)> = vec![entity];
                room.send(&USER_STATUSES::new(refs));
            }
        }
    }
}
