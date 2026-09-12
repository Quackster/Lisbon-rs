//! Mirrors `net.h4bbo.lisbon.game.room.tasks.RainbowTask`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};

use lazy_static::lazy_static;

use crate::game::room::room::Room;

lazy_static! {
    static ref HEX_COLOURS: HashMap<usize, String> = {
        let mut hex_colours = HashMap::new();

        let frequency = 0.5;

        for i in 0..32 {
            let red = (frequency * i as f64 + 0.0).sin() * 127.0 + 128.0;
            let green = (frequency * i as f64 + 2.0).sin() * 127.0 + 128.0;
            let blue = (frequency * i as f64 + 4.0).sin() * 127.0 + 128.0;

            let hex = format!("#{:02x}{:02x}{:02x}", red as i32, green as i32, blue as i32);
            hex_colours.insert(i, hex);
        }

        hex_colours
    };
}

/// Mirrors `RainbowTask` (a `Runnable`).
pub struct RainbowTask {
    room: Room,
    colour_index: AtomicI32,
}

impl RainbowTask {
    /// Mirrors the `RainbowTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            room: room.clone(),
            colour_index: AtomicI32::new(-1),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let moodlight = self.room.get_item_manager().get_moodlight();

        if moodlight.is_none() {
            self.room.get_task_manager().cancel_task("RainbowTask");
            return;
        }

        let colour_index = self.colour_index.fetch_add(1, Ordering::SeqCst) + 1;

        if colour_index < 0 || !HEX_COLOURS.contains_key(&(colour_index as usize)) {
            self.colour_index.store(0, Ordering::SeqCst);
        } else {
            self.colour_index.store(colour_index, Ordering::SeqCst);
        }

        let hex_colour = &HEX_COLOURS[&(self.colour_index.load(Ordering::SeqCst) as usize)];

        // 2,1,1,#0053F7,211
        // enable moodlight, preset id, background state, hex colour, strength (middle)
        if let Some(mut moodlight) = moodlight {
            moodlight.set_custom_data(&format!("2,1,1,{},211", hex_colour));
            moodlight.update_status();
        }
    }
}
