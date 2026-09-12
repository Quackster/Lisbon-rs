//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormAttributes`.
use std::sync::{Arc, atomic::{AtomicI64, AtomicI32, Ordering}};

use parking_lot::Mutex;

use crate::game::game_scheduler::GameScheduler;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::util::snow_storm_activity_state::SnowStormActivityState;
use crate::game::pathfinder::position::Position;

pub struct SnowStormAttributes {
    is_walking: bool,
    current_position: Option<Position>,
    walk_goal: Option<Position>,
    next_goal: Option<Position>,
    goal_world_coordinates: Option<[i32; 2]>,
    snowballs: AtomicI32,
    health: AtomicI32,
    score: AtomicI32,
    rotation: i32,
    last_throw: AtomicI64,
    immunity_expiry: i64,
    // Port note: the Java `activityState` field is never initialised (NPE
    // in the Java `isWalkable` / `isDamageable` until it's set).
    activity_state: Option<SnowStormActivityState>,
    state_time: i64,
}

impl SnowStormAttributes {
    /// Mirrors the `SnowStormAttributes` constructor.
    pub fn new() -> Self {
        Self {
            is_walking: false,
            current_position: None,
            walk_goal: None,
            next_goal: None,
            goal_world_coordinates: None,
            snowballs: AtomicI32::new(0),
            health: AtomicI32::new(0),
            score: AtomicI32::new(0),
            rotation: 0,
            last_throw: AtomicI64::new(0),
            immunity_expiry: 0,
            activity_state: None,
            state_time: 0,
        }
    }

    /// Mirrors `isWalking()`.
    pub fn is_walking(&self) -> bool {
        self.is_walking
    }

    /// Mirrors `setWalking(boolean)`.
    pub fn set_walking(&mut self, walking: bool) {
        self.is_walking = walking
    }

    /// Mirrors `getCurrentPosition()`.
    pub fn get_current_position(&self) -> Option<&Position> {
        self.current_position.as_ref()
    }

    /// Mirrors `setCurrentPosition(Position)`.
    pub fn set_current_position(&mut self, current_position: Option<Position>) {
        self.current_position = current_position
    }

    /// Mirrors `getWalkGoal()`.
    pub fn get_walk_goal(&self) -> Option<&Position> {
        self.walk_goal.as_ref()
    }

    /// Mirrors `setWalkGoal(Position)`.
    pub fn set_walk_goal(&mut self, walk_goal: Option<Position>) {
        self.walk_goal = walk_goal
    }

    /// Mirrors `getNextGoal()`.
    pub fn get_next_goal(&self) -> Option<&Position> {
        self.next_goal.as_ref()
    }

    /// Mirrors `setNextGoal(Position)`.
    pub fn set_next_goal(&mut self, next_goal: Option<Position>) {
        self.next_goal = next_goal
    }

    /// Mirrors `getGoalWorldCoordinates()`.
    pub fn get_goal_world_coordinates(&self) -> Option<[i32; 2]> {
        self.goal_world_coordinates
    }

    /// Mirrors `setGoalWorldCoordinates(int[])`.
    pub fn set_goal_world_coordinates(&mut self, goal_world_coordinates: Option<[i32; 2]>) {
        self.goal_world_coordinates = goal_world_coordinates
    }

    /// Mirrors `getSnowballs().get()` (the Java `AtomicInteger` is an `i32`
    /// snapshot here).
    pub fn get_snowballs(&self) -> i32 {
        self.snowballs.load(Ordering::SeqCst)
    }

    /// Mirrors `getSnowballs().set(int)`.
    pub fn set_snowballs(&self, value: i32) {
        self.snowballs.store(value, Ordering::SeqCst)
    }

    /// Mirrors `getSnowballs().incrementAndGet()`.
    pub fn increment_snowballs(&self) -> i32 {
        self.snowballs.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `getSnowballs().decrementAndGet()`.
    pub fn decrement_snowballs(&self) -> i32 {
        self.snowballs.fetch_sub(1, Ordering::SeqCst) - 1
    }

    /// Mirrors `getHealth().get()` (the Java `AtomicInteger` is an `i32`
    /// snapshot here).
    pub fn get_health(&self) -> i32 {
        self.health.load(Ordering::SeqCst)
    }

    /// Mirrors `getHealth().set(int)`.
    pub fn set_health(&self, value: i32) {
        self.health.store(value, Ordering::SeqCst)
    }

    /// Mirrors `getHealth().incrementAndGet()`.
    pub fn increment_health(&self) -> i32 {
        self.health.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `getHealth().decrementAndGet()`.
    pub fn decrement_health_get(&self) -> i32 {
        self.health.fetch_sub(1, Ordering::SeqCst) - 1
    }

    /// Mirrors `getRotation()`.
    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }

    /// Mirrors `setRotation(int)`.
    pub fn set_rotation(&mut self, rotation: i32) {
        self.rotation = rotation
    }

    /// Mirrors `getActivityState()`.
    pub fn get_activity_state(&self) -> Option<SnowStormActivityState> {
        self.activity_state
    }

    /// Mirrors `isWalkable()`.
    pub fn is_walkable(&self) -> bool {
        // Port note: the Java field is uninitialised (NPE here); `None`
        // counts as not walkable.
        matches!(
            self.activity_state,
            Some(
                SnowStormActivityState::ActivityStateNormal
                    | SnowStormActivityState::ActivityStateInvincibleAfterStun
            )
        )
    }

    /// Mirrors `isDamageable()`.
    pub fn is_damageable(&self) -> bool {
        // Port note: the Java field is uninitialised (NPE here); `None`
        // counts as not `ACTIVITY_STATE_NORMAL`.
        chrono::Utc::now().timestamp_millis() > self.immunity_expiry
            && self.activity_state == Some(SnowStormActivityState::ActivityStateNormal)
    }

    /// Mirrors `setActivityState(SnowStormActivityState, Runnable)` (the
    /// scheduled restore to `ACTIVITY_STATE_NORMAL` is not wired, see the
    /// module note).
    /// Sets the activity state directly (the scheduled restore's
    // `this.activityState = ACTIVITY_STATE_NORMAL`); it does not touch
    // `state_time` nor schedule anything.
    pub fn set_activity_state_raw(&mut self, activity_state: SnowStormActivityState) {
        self.activity_state = Some(activity_state);
    }

    /// Mirrors `setActivityState(SnowStormActivityState, Runnable)`.
    pub fn set_activity_state(
        &mut self,
        player: Arc<Mutex<GamePlayer>>,
        activity_state: SnowStormActivityState,
        callback: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) {
        self.activity_state = Some(activity_state);
        self.state_time = chrono::Utc::now().timestamp_millis() + activity_state.get_time_in_ms() as i64;

        if activity_state != SnowStormActivityState::ActivityStateNormal {
            GameScheduler::get_instance().schedule(
                {
                    let player = Arc::clone(&player);
                    move || {
                        {
                            let mut p = player.lock();
                            p.get_snow_storm_attributes_mut()
                                .set_activity_state_raw(SnowStormActivityState::ActivityStateNormal);
                        }

                        if let Some(callback) = callback {
                            callback();
                        }
                    }
                },
                activity_state.get_time_in_ms() as i64,
            );
        }
    }

    /// Mirrors `getActivityTimer()`.
    pub fn get_activity_timer(&self) -> i32 {
        // Port note: the Java `activityState` is uninitialised (NPE here);
        // `None` returns `0`.
        let Some(activity_state) = self.activity_state else {
            return 0;
        };

        let mut time_remaining: i32 = 0;
        let now = chrono::Utc::now().timestamp_millis();
        let expire_time = self.state_time + activity_state.get_time_in_ms() as i64;

        if !(now > self.state_time
            || activity_state == SnowStormActivityState::ActivityStateNormal)
        {
            time_remaining = (expire_time - now) as i32;
            time_remaining = (time_remaining / 300) * 5;
        }

        time_remaining
    }

    /// Mirrors `getScore().get()` (the Java `AtomicInteger` is an `i32`
    /// snapshot here).
    pub fn get_score(&self) -> i32 {
        self.score.load(Ordering::SeqCst)
    }

    /// Mirrors `setScore(AtomicInteger)`.
    pub fn set_score(&self, score: i32) {
        self.score.store(score, Ordering::SeqCst)
    }

    /// Mirrors `getScore().incrementAndGet()`.
    pub fn increment_score(&self) -> i32 {
        self.score.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `getScore().addAndGet(int)`.
    pub fn add_score(&self, delta: i32) -> i32 {
        self.score.fetch_add(delta, Ordering::SeqCst) + delta
    }

    /// Mirrors `getLastThrow().get()` (the Java `AtomicLong` is an `i64`
    /// snapshot here).
    pub fn get_last_throw(&self) -> i64 {
        self.last_throw.load(Ordering::SeqCst)
    }

    /// Mirrors `getLastThrow().set(long)`.
    pub fn set_last_throw(&self, last_throw: i64) {
        self.last_throw.store(last_throw, Ordering::SeqCst)
    }

    /// Mirrors `getImmunityExpiry()`.
    pub fn get_immunity_expiry(&self) -> i64 {
        self.immunity_expiry
    }

    /// Mirrors `setImmunityExpiry(long)`.
    pub fn set_immunity_expiry(&mut self, immunity_expiry: i64) {
        self.immunity_expiry = immunity_expiry
    }
}

impl Default for SnowStormAttributes {
    fn default() -> Self {
        Self::new()
    }
}
