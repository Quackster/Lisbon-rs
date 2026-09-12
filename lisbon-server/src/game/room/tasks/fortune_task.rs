//! Mirrors `net.h4bbo.lisbon.game.room.tasks.FortuneTask`.
use rand::Rng;

use crate::game::item::item::Item;

/// Mirrors `FortuneTask` (the Java `Runnable` is the `run` method).
pub struct FortuneTask {
    fortune: Item,
}

impl FortuneTask {
    /// Mirrors the `FortuneTask(Item)` constructor.
    pub fn new(item: Item) -> Self {
        Self { fortune: item }
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        if !self.fortune.get_requires_update() {
            return;
        }

        let random_number = rand::thread_rng().gen_range(1..=10);

        self.fortune.set_custom_data(&random_number.to_string());
        self.fortune.update_status();
        self.fortune.set_requires_update(false);
        self.fortune.save();
    }
}
