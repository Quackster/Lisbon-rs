//! Mirrors `net.h4bbo.lisbon.game.room.tasks.DiceTask`.
use rand::Rng;

use crate::game::item::item::Item;
use crate::messages::outgoing::rooms::items::dice_value::DICE_VALUE;

/// Mirrors `DiceTask` (the Java `Runnable` is the `run` method).
pub struct DiceTask {
    dice: Item,
}

impl DiceTask {
    /// Mirrors the `DiceTask(Item)` constructor.
    pub fn new(dice: Item) -> Self {
        Self { dice }
    }

    /// Mirrors `run()`.
    pub fn run(&mut self) {
        if !self.dice.get_requires_update() {
            return;
        }

        let mut max_number = 6;

        if self.dice.get_definition().get_sprite() == "bottle" {
            max_number = 8;
        }

        let random_number = rand::thread_rng().gen_range(1..=max_number);

        if let Some(room) = self.dice.get_room() {
            room.lock().send(&DICE_VALUE::new(self.dice.get_id(), false, random_number));
        }

        self.dice.set_custom_data(&random_number.to_string());
        self.dice.update_status();
        self.dice.set_requires_update(false);
        self.dice.save();
    }
}
