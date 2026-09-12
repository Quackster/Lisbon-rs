//! Mirrors `net.h4bbo.lisbon.game.room.tasks.BotTask`.
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::room::room::Room;
use crate::util::date_util::DateUtil;

/// Mirrors `BotTask` (the Java `Runnable` is the `run` method).
pub struct BotTask {
    room: Room,
    min_walk_time: i32,
    max_walk_time: i32,
    min_speak_time: i32,
    max_speak_time: i32,
    last_speech: parking_lot::Mutex<String>,
}

impl BotTask {
    /// Mirrors the `BotTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        let now = DateUtil::get_current_time_seconds() as i64;

        for mut bot in room.get_entity_manager().get_bots() {
            bot.set_next_walk_time(now + Self::random_in(3, 10));
            bot.set_next_speech_time(now + Self::random_in(20, 50));
        }

        Self {
            room: room.clone(),
            min_walk_time: 3,
            max_walk_time: 10,
            min_speak_time: 20,
            max_speak_time: 50,
            last_speech: parking_lot::Mutex::new(String::new()),
        }
    }

    /// Mirrors the Java `ThreadLocalRandom.current().nextInt(min, max)`
    /// (an inclusive `min` and exclusive `max` draw).
    fn random_in(min: i32, max: i32) -> i64 {
        rand::thread_rng().gen_range(min..max) as i64
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let now = DateUtil::get_current_time_seconds() as i64;

        for mut bot in self.room.get_entity_manager().get_bots() {
            if now > bot.get_next_walk_time() {
                if let Some(bot_data) = bot.get_bot_data() {
                    let walkspace = bot_data.get_walkspace();

                    if !walkspace.is_empty() {
                        let walk_destination =
                            &walkspace[rand::thread_rng().gen_range(0..walkspace.len())];
                        if let Some(room_user) = bot.get_room_user() {
                            room_user.walk_to(walk_destination.get_x(), walk_destination.get_y());
                        }
                        bot.set_next_walk_time(
                            now + Self::random_in(self.min_walk_time, self.max_walk_time),
                        );
                    }
                }
            }

            if now > bot.get_next_speech_time() {
                if let Some(bot_data) = bot.get_bot_data() {
                    let speeches = bot_data.get_speeches();

                    if !speeches.is_empty() {
                        let speech = &speeches[rand::thread_rng().gen_range(0..speeches.len())];
                        // The `BotSpeech` fields are copied out before the
                        // `next_speech_time` mutation (the Java reference
                        // compare `this.lastSpeech != speech` becomes a
                        // speech-text compare since `BotSpeech` has no
                        // `PartialEq`).
                        let speech_text = speech.get_speech().to_string();
                        let chat_message_type = speech.get_chat_message_type();

                        bot.set_next_speech_time(
                            now + Self::random_in(self.min_speak_time, self.max_speak_time),
                        );

                        if *self.last_speech.lock() != speech_text {
                            if let Some(room_user) = bot.get_room_user() {
                                room_user.talk(&speech_text, chat_message_type);
                            }
                        }

                        *self.last_speech.lock() = speech_text;
                    }
                }
            }
        }
    }
}

impl crate::game::room::managers::room_task_manager::Tickable for BotTask {
    fn tick(&self) {
        self.run();
    }
}
