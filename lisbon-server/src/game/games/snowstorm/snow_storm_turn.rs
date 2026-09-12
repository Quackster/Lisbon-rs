//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.SnowStormTurn`.
use std::sync::Arc;

use crate::game::games::game_object::GameObject;

#[derive(Clone)]
pub struct SnowStormTurn {
    events: Vec<Arc<dyn GameObject>>,
}

impl SnowStormTurn {
    /// Mirrors the `SnowStormTurn()` constructor (the
    /// `CopyOnWriteArrayList` is a plain `Vec`).
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Mirrors `getSubTurns()`.
    pub fn get_sub_turns(&self) -> &Vec<Arc<dyn GameObject>> {
        &self.events
    }

    /// Mirrors the `List` `add` calls on `getSubTurns()`.
    pub fn add_sub_turn(&mut self, event: Arc<dyn GameObject>) {
        self.events.push(event);
    }
}
