//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormFuture`.
use std::sync::Arc;

use crate::game::games::game_object::GameObject;

pub struct SnowStormFuture {
    frames_future: i32,
    sub_turn: i32,
    event: Arc<dyn GameObject>,
}

impl SnowStormFuture {
    /// Mirrors the `SnowStormFuture(int, int, GameObject)` constructor.
    pub fn new(
        frames_future: i32,
        sub_turn: i32,
        event: Arc<dyn GameObject>,
    ) -> Self {
        Self {
            frames_future,
            sub_turn,
            event,
        }
    }

    /// Mirrors `getFramesFuture()`.
    pub fn get_frames_future(&self) -> i32 {
        self.frames_future
    }

    /// Mirrors `getSubTurn()`.
    pub fn get_sub_turn(&self) -> i32 {
        self.sub_turn
    }

    /// Mirrors `getEvent()`.
    pub fn get_event(&self) -> &Arc<dyn GameObject> {
        &self.event
    }

    /// Mirrors `decrementFrame()`.
    pub fn decrement_frame(&mut self) {
        self.frames_future -= 1;
    }

    /// Consume the shared event (the Java shared reference is handed
    /// over by value in Rust).
    pub fn into_event(self) -> Arc<dyn GameObject> {
        self.event
    }
}
