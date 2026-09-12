//! Mirrors `net.h4bbo.lisbon.game.room.tasks.SpaceCafeTask`.
use rand::Rng;

use parking_lot::Mutex;

use crate::game::room::managers::room_task_manager::Tickable;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;

/// The mutable tick state (the Java fields the `Runnable` mutates on the
/// scheduler thread; the registry shares the task, so the state is behind a
/// `Mutex`).
struct SpaceCafeTaskState {
    stage_index: i32,
    time_to_next_stage: i64,
    time_to_next_light: i64,
    first_colour: i32,
    second_colour: i32,
    third_colour: i32,
}

/// Mirrors `SpaceCafeTask` (the Java `Runnable` is the `run` method).
pub struct SpaceCafeTask {
    room: Room,
    state: Mutex<SpaceCafeTaskState>,
}

impl SpaceCafeTask {
    /// Mirrors the `FLIPBOARD_TIME` constant.
    pub const FLIPBOARD_TIME: i32 = 60;

    /// Mirrors the `LIGHT_TIME` constant.
    pub const LIGHT_TIME: i32 = 5;

    /// Mirrors the `FLIPBOARD_ORDER` constant.
    const FLIPBOARD_ORDER: [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    /// Mirrors the `SpaceCafeTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            room: room.clone(),
            state: Mutex::new(SpaceCafeTaskState {
                stage_index: 2,
                time_to_next_stage: Self::convert_to_interval(Self::FLIPBOARD_TIME) as i64,
                time_to_next_light: Self::convert_to_interval(Self::LIGHT_TIME) as i64,
                first_colour: 0,
                second_colour: 0,
                third_colour: 0,
            }),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let mut state = self.state.lock();

        if state.time_to_next_stage > 0 {
            state.time_to_next_stage -= 1;
        }

        if state.time_to_next_stage == 0 {
            Self::process_board_flip(&mut state, &self.room);
        }

        if state.time_to_next_light > 0 {
            state.time_to_next_light -= 1;
        }

        if state.time_to_next_light == 0 {
            Self::process_light_color(&mut state, &self.room);
        }
    }

    /// Mirrors `processLightColor()`.
    fn process_light_color(state: &mut SpaceCafeTaskState, room: &Room) {
        let numbers = [4, 1, 2];

        let temp_first_colour = Self::find_new_colour(state.first_colour, &numbers);
        let temp_second_colour = Self::find_new_colour(state.second_colour, &numbers);
        let temp_third_colour = Self::find_new_colour(state.third_colour, &numbers);

        if temp_first_colour != state.first_colour {
            state.first_colour = temp_first_colour;
            room.send(&SHOWPROGRAM::new(vec![
                "lmpa".to_string(),
                "ufol".to_string(),
                state.first_colour.to_string(),
            ]));
            room.send(&SHOWPROGRAM::new(vec![
                "liga".to_string(),
                "litecol".to_string(),
                state.first_colour.to_string(),
            ]));
        }

        if temp_second_colour != state.second_colour {
            state.second_colour = temp_second_colour;
            room.send(&SHOWPROGRAM::new(vec![
                "lmpb".to_string(),
                "ufol".to_string(),
                state.second_colour.to_string(),
            ]));
            room.send(&SHOWPROGRAM::new(vec![
                "ligb".to_string(),
                "litecol".to_string(),
                state.second_colour.to_string(),
            ]));
        }

        if temp_third_colour != state.third_colour {
            state.third_colour = temp_third_colour;
            room.send(&SHOWPROGRAM::new(vec![
                "lmpc".to_string(),
                "ufol".to_string(),
                state.third_colour.to_string(),
            ]));
            room.send(&SHOWPROGRAM::new(vec![
                "ligc".to_string(),
                "litecol".to_string(),
                state.third_colour.to_string(),
            ]));
        }

        state.time_to_next_light = Self::convert_to_interval(Self::LIGHT_TIME) as i64;
    }

    /// Mirrors `processBoardFlip()`.
    fn process_board_flip(state: &mut SpaceCafeTaskState, room: &Room) {
        state.stage_index += 1;

        if state.stage_index >= Self::FLIPBOARD_ORDER.len() as i32 {
            state.stage_index = 0;
        }

        let previous_board = state.stage_index;
        let next_board = if (previous_board + 1) >= Self::FLIPBOARD_ORDER.len() as i32 {
            0
        } else {
            previous_board + 1
        };

        room.send(&SHOWPROGRAM::new(vec![
            format!("flipflop{}", Self::FLIPBOARD_ORDER[previous_board as usize]),
            "visible".to_string(),
            "0".to_string(),
        ]));
        room.send(&SHOWPROGRAM::new(vec![
            format!("flipflop{}", Self::FLIPBOARD_ORDER[next_board as usize]),
            "visible".to_string(),
            "1".to_string(),
        ]));

        let is_still_board = (previous_board == 2 && next_board == 3)
            || (previous_board == 5 && next_board == 6)
            || (previous_board == 8 && next_board == 0);

        if is_still_board {
            state.time_to_next_stage = Self::convert_to_interval(Self::FLIPBOARD_TIME) as i64;
        } else {
            state.time_to_next_stage = 1;
        }
    }

    /// Mirrors `findNewColour(int, int[])` (the Java "different colour"
    // loop is commented out in the Java source).
    pub fn find_new_colour(_current_colour: i32, selection: &[i32]) -> i32 {
        selection[rand::thread_rng().gen_range(0..selection.len())]
    }

    /// Mirrors `convertToInterval(int)`.
    pub fn convert_to_interval(seconds: i32) -> i32 {
        seconds * 2
    }
}

impl Tickable for SpaceCafeTask {
    fn tick(&self) {
        self.run();
    }
}
