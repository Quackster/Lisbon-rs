//! Mirrors `net.h4bbo.lisbon.game.room.managers.RoomTaskManager`.
//!
//! Each `Room` owns its manager (the Java `new RoomTaskManager(this)`);
//! the per-room fixed-rate tasks run on a dedicated blocking thread per
//! task (mirroring the Java `ScheduledExecutorService
//! .scheduleAtFixedRate`), and the Java `ScheduledFuture.cancel(false)`
//! maps to the cancel flag.

use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_task::BattleBallTask;
use crate::game::games::snowstorm::tasks::snow_storm_game_task::SnowStormGameTask;
use crate::game::room::room::Room;
use crate::game::room::tasks::entity_task::EntityTask;
use crate::game::room::tasks::rainbow_task::RainbowTask;
use crate::game::room::tasks::roller_task::RollerTask;
use crate::game::room::tasks::status_task::StatusTask;
use crate::util::config::game_configuration::GameConfiguration;

/// A fixed-rate room task (the Java `Runnable` in the
/// `Pair<ScheduledFuture, Runnable>` registry).
pub trait Tickable: Send + Sync {
    fn tick(&self);
}

impl Tickable for EntityTask {
    fn tick(&self) {
        self.run();
    }
}

impl Tickable for StatusTask {
    fn tick(&self) {
        self.run();
    }
}

impl Tickable for RollerTask {
    fn tick(&self) {
        self.run();
    }
}

impl Tickable for BattleBallTask {
    fn tick(&self) {
        self.run();
    }
}

impl Tickable for SnowStormGameTask {
    fn tick(&self) {
        self.run();
    }
}

impl Tickable for RainbowTask {
    fn tick(&self) {
        self.run();
    }
}

/// The per-room registry of fixed-rate tasks.
pub struct RoomTaskManager {
    tasks: Mutex<HashMap<String, (Arc<AtomicBool>, Arc<dyn Any + Send + Sync>)>>,
}

impl RoomTaskManager {
    /// Mirrors the `RoomTaskManager(Room)` constructor (the room is not
    /// stored; `start_tasks` takes it as a parameter).
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
        }
    }

    /// Mirrors `startTasks()`.
    pub fn start_tasks(&self, room: &Room) {
        let model_name = room
            .get_model()
            .map(|model| model.get_name().to_string())
            .unwrap_or_default();

        if model_name.contains("_arena_") || model_name.contains("snowwar_arena") {
            self.load_game_tasks(room);
            return;
        }

        self.schedule_task("EntityTask", Arc::new(EntityTask::new(room)), 0, 500);
        self.schedule_task("StatusTask", Arc::new(StatusTask::new(room)), 0, 1000);

        if !room.is_public_room() {
            let roller_millis =
                GameConfiguration::get_instance().get_integer("roller.tick.default") as i64;
            self.schedule_task("RollerTask", Arc::new(RollerTask::new(room)), 250, roller_millis);
        }
    }

    /// Mirrors `loadGameTasks()`.
    fn load_game_tasks(&self, room: &Room) {
        let Some(game) = room.get_data().get_game().cloned() else {
            return;
        };

        if game.as_battle_ball().is_some() {
            self.schedule_task(
                "GameTask",
                Arc::new(BattleBallTask::new(room.clone(), game.clone())),
                0,
                500,
            );
        }

        if let Some(snow_storm) = game.as_snow_storm() {
            self.schedule_task(
                "UpdateTask",
                Arc::new(SnowStormGameTask::new(room.clone(), snow_storm)),
                0,
                300,
            );
        }
    }

    /// Mirrors `stopTasks()`.
    pub fn stop_tasks(&self) {
        let mut tasks = self.tasks.lock();
        for (_, (cancel, _)) in tasks.iter() {
            cancel.store(true, Ordering::SeqCst);
        }
        tasks.clear();
    }

    /// Mirrors `scheduleTask(String, Runnable, long, int, TimeUnit)` (the
    /// delays are in milliseconds).
    pub fn schedule_task<T: Tickable + Any + Send + Sync + 'static>(
        &self,
        name: &str,
        task: Arc<T>,
        initial_delay_ms: i64,
        period_ms: i64,
    ) {
        self.cancel_task(name);

        let cancel = Arc::new(AtomicBool::new(false));
        let task_arc = Arc::clone(&task);
        let handle: Arc<dyn Any + Send + Sync> = task_arc.clone();
        let runner: Arc<dyn Tickable> = task_arc;
        let task_cancel = Arc::clone(&cancel);

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(initial_delay_ms.max(0) as u64));
            while !task_cancel.load(Ordering::SeqCst) {
                runner.tick();
                thread::sleep(Duration::from_millis(period_ms.max(0) as u64));
            }
        });

        self.tasks
            .lock()
            .insert(name.to_string(), (cancel, handle));
    }

    /// Mirrors `cancelTask(String)`.
    pub fn cancel_task(&self, name: &str) {
        if let Some((cancel, _)) = self.tasks.lock().remove(name) {
            cancel.store(true, Ordering::SeqCst);
        }
    }

    /// Mirrors `getTask(String)` (the Java `Runnable` cast maps to the
    /// `Arc` downcast at the call site).
    pub fn get_task(&self, name: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        self.tasks.lock().get(name).map(|(_, handle)| Arc::clone(handle))
    }

    /// Mirrors the Java `(ConcreteTask) getTask(String)` cast.
    pub fn get_task_as<T: Any + Send + Sync + 'static>(&self, name: &str) -> Option<Arc<T>> {
        self.get_task(name)?.downcast::<T>().ok()
    }

    /// Mirrors `hasTask(String)`.
    pub fn has_task(&self, name: &str) -> bool {
        self.tasks.lock().contains_key(name)
    }
}
