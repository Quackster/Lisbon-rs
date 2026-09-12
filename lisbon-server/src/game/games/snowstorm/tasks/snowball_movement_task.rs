//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.tasks.SnowballMovementTask`.
//! The fixed-rate loop (`scheduleAtFixedRate`) is driven by
//! `SnowballObject::scheduleMovementTask`; `endMovement`'s
//! `cancelFuture` stops it through the shared `Arc<AtomicBool>` token.
use std::collections::VecDeque;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::snowstorm::mapping::snow_storm_pathfinder::SnowStormPathfinder;
use crate::game::games::snowstorm::objects::snowball_object::{SnowballObject, SnowballTrajectory};
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::pathfinder::position::Position;
use crate::util::schedule::future_runnable::FutureRunnable;

pub struct SnowballMovementTask {
    future_runnable: FutureRunnable,
    snowball: Arc<Mutex<SnowballObject>>,
    first_position: Position,
    path: VecDeque<Position>,
    thrown_time: i64,
    last_position: Position,
}

impl SnowballMovementTask {
    /// Mirrors the `SnowballMovementTask(SnowballObject)` constructor.
    pub fn new(snowball: Arc<Mutex<SnowballObject>>) -> Self {
        let (direction, from_x, from_y, path, time_to_live) = {
            let guard = snowball.lock();
            (
                guard.get_direction(),
                guard.get_from_x(),
                guard.get_from_y(),
                guard.get_path(),
                guard.get_time_to_live(),
            )
        };

        Self {
            future_runnable: FutureRunnable::new(),
            first_position: Position::new(from_x, from_y, direction as f64),
            path: VecDeque::from(path),
            thrown_time: chrono::Utc::now().timestamp_millis() + time_to_live as i64 * 100,
            last_position: Position::new(from_x, from_y, direction as f64),
            snowball,
        }
    }

    /// Mirrors `getFuture()`.
    pub fn get_future(&self) -> Option<Arc<std::sync::atomic::AtomicBool>> {
        self.future_runnable.get_future()
    }

    /// Mirrors `setFuture(Future)`.
    pub fn set_future(&mut self, future: Arc<std::sync::atomic::AtomicBool>) {
        self.future_runnable.set_future(future)
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        if self.snowball.lock().get_trajectory() != Some(SnowballTrajectory::LongTrajectory) {
            if !self.path.is_empty() {
                let next_position = self.path.pop_front().unwrap();

                self.last_position.set_x(next_position.get_x());
                self.last_position.set_y(next_position.get_y());

                if SnowStormPathfinder::is_blocked_tile(&self.snowball.lock(), &next_position) {
                    let mut snowball = self.snowball.lock();
                    snowball.set_target_x(next_position.get_x());
                    snowball.set_target_y(next_position.get_y());
                    snowball.set_blocked(true);
                    drop(snowball);

                    self.end_movement(true);
                    return;
                }
            } else {
                // "Finished flying"
                if (self.snowball.lock().get_trajectory() == Some(SnowballTrajectory::QuickThrow)
                    || self.snowball.lock().get_trajectory() == Some(SnowballTrajectory::LongTrajectory))
                    && self.continue_quick_throw_snowball()
                {
                    self.path = VecDeque::from(self.snowball.lock().get_path());
                } else {
                    self.end_movement(true);
                }

                return;
            }
        }

        if chrono::Utc::now().timestamp_millis() > self.thrown_time {
            if self.snowball.lock().get_trajectory() == Some(SnowballTrajectory::QuickThrow)
                && self.continue_quick_throw_snowball()
            {
                self.path = VecDeque::from(self.snowball.lock().get_path());
            } else {
                self.end_movement(true);
            }
        }
    }

    /// Mirrors `continueQuickThrowSnowball()`.
    fn continue_quick_throw_snowball(&mut self) -> bool {
        let target_player = self.snowball.lock().get_target_player().cloned();

        // Port note: the Java `NullPointerException`s if the
        // `targetPlayer` is unset; the scan is aborted instead.
        let Some(target_player) = target_player else {
            return false;
        };

        let target_player_id = target_player
            .lock()
            .get_player()
            .lock()
            .get_details()
            .get_id();

        // Only check in straight lines... otherwise it's a goddamn heat
        // seeking missile
        let mut next_position = Position::with_rotations(
            self.snowball.lock().get_from_x(),
            self.snowball.lock().get_from_y(),
            0.0,
            self.snowball.lock().get_direction(),
            self.snowball.lock().get_direction(),
        );

        let game = self.snowball.lock().get_game().clone();

        loop {
            let map = game.get_map();
            let Some(map) = map else {
                break;
            };
            let Some(_tile) = map.get_tile(&next_position) else {
                break;
            };

            let players = game.get_active_players();
            let opposition_player = players.iter().find(|p| {
                let guard = p.lock();
                let attrs = guard.get_snow_storm_attributes();
                attrs
                    .get_current_position()
                    .map(|c| c == &next_position)
                    .unwrap_or(false)
                    && guard
                        .get_player()
                        .lock()
                        .get_details()
                        .get_id()
                        == target_player_id
                    && attrs.is_damageable()
            });

            let Some(opposition_player) = opposition_player else {
                next_position = next_position.get_square_in_front();
                continue;
            };

            let distance = opposition_player
                .lock()
                .get_snow_storm_attributes()
                .get_current_position()
                .map(|c| c.get_distance_squared(&self.first_position))
                .unwrap_or(0);

            if distance >= SnowStormGame::MAX_QUICK_THROW_DISTANCE {
                return false;
            }

            let target_position = opposition_player
                .lock()
                .get_snow_storm_attributes()
                .get_current_position()
                .cloned()
                .unwrap_or_default();

            {
                let mut snowball = self.snowball.lock();
                snowball.set_from_x(self.last_position.get_x());
                snowball.set_from_y(self.last_position.get_y());
                snowball.set_target_x(target_position.get_x());
                snowball.set_target_y(target_position.get_y());
            }

            self.thrown_time = chrono::Utc::now().timestamp_millis()
                + self.snowball.lock().get_time_to_live() as i64 * 100;

            return true;
        }

        false
    }

    /// Mirrors `endMovement(SnowballObject, boolean)`.
    fn end_movement(&mut self, delete_after_hit: bool) {
        let game = self.snowball.lock().get_game().clone();
        game.handle_snowball_landing(&self.snowball, delete_after_hit);
        self.future_runnable.cancel_future();
    }
}
