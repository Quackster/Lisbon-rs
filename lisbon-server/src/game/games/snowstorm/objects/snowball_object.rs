//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.objects.SnowballObject`.
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::mapping::snow_storm_pathfinder::SnowStormPathfinder;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::tasks::snowball_movement_task::SnowballMovementTask;
use crate::game::pathfinder::position::Position;

/// Mirrors the nested `SnowballObject.SnowballTrajectory` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnowballTrajectory {
    QuickThrow,
    ShortTrajectory,
    LongTrajectory,
}

impl SnowballTrajectory {
    /// Mirrors `getTrajectoryId()`.
    pub fn get_trajectory_id(&self) -> i32 {
        match self {
            Self::QuickThrow => 0,
            Self::ShortTrajectory => 1,
            Self::LongTrajectory => 2,
        }
    }

    /// Mirrors `getVelocity()`.
    pub fn get_velocity(&self) -> i32 {
        3000
    }
}

pub struct SnowballObject {
    object_id: i32,
    game: Arc<SnowStormGame>,
    thrower: Arc<Mutex<GamePlayer>>,
    from_x: i32,
    from_y: i32,
    target_x: i32,
    target_y: i32,
    trajectory: i32,
    direction: i32,
    is_blocked: bool,
    target_player: Option<Arc<Mutex<GamePlayer>>>,
}

impl SnowballObject {
    /// Mirrors the `SnowballObject(int, SnowStormGame, GamePlayer, int,
    /// int, int, int, int, int)` constructor.
    pub fn new(
        object_id: i32,
        snow_storm_game: Arc<SnowStormGame>,
        thrower: Arc<Mutex<GamePlayer>>,
        from_x: i32,
        from_y: i32,
        target_x: i32,
        target_y: i32,
        trajectory: i32,
        direction: i32,
    ) -> Self {
        Self {
            object_id,
            game: snow_storm_game,
            thrower,
            from_x,
            from_y,
            target_x,
            target_y,
            trajectory,
            direction,
            is_blocked: false,
            target_player: None,
        }
    }

    /// Mirrors `getTargetPlayer()`.
    pub fn get_target_player(&self) -> Option<&Arc<Mutex<GamePlayer>>> {
        self.target_player.as_ref()
    }

    /// Mirrors `setTargetPlayer(GamePlayer)`.
    pub fn set_target_player(&mut self, target_player: Arc<Mutex<GamePlayer>>) {
        self.target_player = Some(target_player);
    }

    /// Mirrors `getTimeToLive()`.
    pub fn get_time_to_live(&self) -> i32 {
        let t_x = SnowStormGame::convert_to_world_coordinate(self.from_x);
        let t_y = SnowStormGame::convert_to_world_coordinate(self.from_y);

        let t_delta_x = (SnowStormGame::convert_to_world_coordinate(self.target_x) - t_x) / 200;
        let t_delta_y = (SnowStormGame::convert_to_world_coordinate(self.target_y) - t_y) / 200;

        let t_distance_to_target =
            ((t_delta_x * t_delta_x + t_delta_y * t_delta_y) as f64).sqrt() * 200.0;

        let velocity = self.get_trajectory().map(|t| t.get_velocity()).unwrap_or(0);

        if velocity == 0 {
            return -1;
        }

        (t_distance_to_target / velocity as f64) as i32
    }

    /// Mirrors `getPath()`.
    pub fn get_path(&self) -> Vec<Position> {
        SnowStormPathfinder::get_max_visibility(
            self,
            self.get_from_x(),
            self.get_from_y(),
            self.get_target_x(),
            self.get_target_y(),
            self.get_trajectory(),
        )
    }

    /// Mirrors `scheduleMovementTask()`.
    pub fn schedule_movement_task(this: &Arc<Mutex<SnowballObject>>) {
        let path = this.lock().get_path();

        if this.lock().get_time_to_live() > 0 && !path.is_empty() {
            let mut future_runnable = SnowballMovementTask::new(Arc::clone(this));

            // Mirrors the Java `scheduleAtFixedRate(futureRunnable, 0,
            // (timeToLive * 100) / path.size(), MS)` (the `FutureRunnable`
            // cancellation token stops the loop when the movement ends).
            let cancel = Arc::new(AtomicBool::new(false));
            future_runnable.set_future(Arc::clone(&cancel));
            let period =
                (this.lock().get_time_to_live() as i64 * 100) / path.len() as i64;

            thread::spawn(move || {
                while !cancel.load(Ordering::SeqCst) {
                    future_runnable.run();
                    thread::sleep(Duration::from_millis(period.max(1) as u64));
                }
            });
        } else {
            let game = this.lock().get_game().clone();
            game.handle_snowball_landing(this, false);
        }
    }

    /// Mirrors `getObjectId()`.
    pub fn get_object_id(&self) -> i32 {
        self.object_id
    }

    /// Mirrors `getThrower()`.
    pub fn get_thrower(&self) -> &Arc<Mutex<GamePlayer>> {
        &self.thrower
    }

    /// Mirrors `setThrower(GamePlayer)`.
    pub fn set_thrower(&mut self, thrower: Arc<Mutex<GamePlayer>>) {
        self.thrower = thrower;
    }

    /// Mirrors `getGame()`.
    pub fn get_game(&self) -> &Arc<SnowStormGame> {
        &self.game
    }

    /// Mirrors `getFromX()`.
    pub fn get_from_x(&self) -> i32 {
        self.from_x
    }

    /// Mirrors `setFromX(int)`.
    pub fn set_from_x(&mut self, from_x: i32) {
        self.from_x = from_x
    }

    /// Mirrors `getFromY()`.
    pub fn get_from_y(&self) -> i32 {
        self.from_y
    }

    /// Mirrors `setFromY(int)`.
    pub fn set_from_y(&mut self, from_y: i32) {
        self.from_y = from_y
    }

    /// Mirrors `getTargetX()`.
    pub fn get_target_x(&self) -> i32 {
        self.target_x
    }

    /// Mirrors `setTargetX(int)`.
    pub fn set_target_x(&mut self, target_x: i32) {
        self.target_x = target_x
    }

    /// Mirrors `getTargetY()`.
    pub fn get_target_y(&self) -> i32 {
        self.target_y
    }

    /// Mirrors `setTargetY(int)`.
    pub fn set_target_y(&mut self, target_y: i32) {
        self.target_y = target_y
    }

    /// Mirrors `getTrajectory()`.
    pub fn get_trajectory(&self) -> Option<SnowballTrajectory> {
        if self.trajectory == SnowballTrajectory::QuickThrow.get_trajectory_id() {
            return Some(SnowballTrajectory::QuickThrow);
        }

        if self.trajectory == SnowballTrajectory::ShortTrajectory.get_trajectory_id() {
            return Some(SnowballTrajectory::ShortTrajectory);
        }

        if self.trajectory == SnowballTrajectory::LongTrajectory.get_trajectory_id() {
            return Some(SnowballTrajectory::LongTrajectory);
        }

        None
    }

    /// Mirrors `setTrajectory(int)`.
    pub fn set_trajectory(&mut self, trajectory: i32) {
        self.trajectory = trajectory
    }

    /// Mirrors `isBlocked()`.
    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    /// Mirrors `setBlocked(boolean)`.
    pub fn set_blocked(&mut self, blocked: bool) {
        self.is_blocked = blocked
    }

    /// Mirrors `getDirection()`.
    pub fn get_direction(&self) -> i32 {
        self.direction
    }
}
