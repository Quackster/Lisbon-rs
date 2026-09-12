//! Mirrors `net.h4bbo.lisbon.game.bot.BotManager`.

use std::sync::OnceLock;

use rand::Rng;

use crate::dao::mysql::bot_dao::BotDao;
use crate::game::bot::bot::Bot;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::util::config::game_configuration::GameConfiguration;

static INSTANCE: OnceLock<BotManager> = OnceLock::new();

pub struct BotManager;

impl BotManager {
    /// Mirrors `addBots(Room)`.
    pub fn add_bots(&self, room: &Room) {
        if !GameConfiguration::get_instance().get_bool("room.bots.enabled") {
            return;
        }

        let bot_data_list = BotDao::get_bot_data(room.get_id());

        for bot_data in &bot_data_list {
            let mut bot = Bot::with_data(bot_data.clone());
            bot.get_details_mut()
                .fill_brief(0, bot_data.get_name(), bot_data.get_figure(), bot_data.get_mission(), "M");

            let mut start_position = bot_data.get_start_position().copy();
            start_position.set_z(match room
                .get_mapping().lock()
                .get_tile(
                    room,
                    bot_data.get_start_position().get_x(),
                    bot_data.get_start_position().get_y(),
                ) {
                Some(tile) => tile.get_walking_height(),
                None => 0.0,
            });

            room.get_entity_manager().enter_room(room, &bot, Some(&start_position));
        }

        if !bot_data_list.is_empty() {
            room.get_task_manager().schedule_task(
                "BotCommandTask",
                std::sync::Arc::new(crate::game::room::tasks::bot_task::BotTask::new(room)),
                0,
                1000,
            );
        }
    }

    /// Mirrors `handleSpeech(Player, Room, String)`.
    pub fn handle_speech(&self, player: &Player, room: &Room, message: &str) {
        let mut bots: Vec<Box<Bot>> = Vec::new();

        for bot in room.get_entity_manager().get_bots() {
            if bot
                .get_room_user()
                .expect("bot must have a room user")
                .get_position()
                .get_distance_squared(
                    &player
                        .get_room_user()
                        .expect("player must have a room user")
                        .get_position(),
                ) > 14
            {
                continue;
            }

            if bot.get_bot_data().is_none() {
                continue;
            }

            bots.push(bot);
        }

        for bot in &bots {
            if let Some(drink) = self.has_requested_drink(player, bot, message) {
                if let Some(bot_data) = bot.get_bot_data() {
                    if !bot_data.get_responses().is_empty() {
                        let index = rand::thread_rng().gen_range(0..bot_data.get_responses().len());
                        let bot_speech = &bot_data.get_responses()[index];
                        let mut chat_message = bot_speech.get_speech().to_string();

                        chat_message = chat_message.replace("%lowercaseDrink%", &drink.to_lowercase());
                        chat_message = chat_message.replace("%drink%", &drink);

                        bot.get_room_user()
                            .expect("bot must have a room user")
                            .talk(&chat_message, bot_speech.get_chat_message_type());
                    }
                }
                continue;
            }

            if let Some(bot_data) = bot.get_bot_data() {
                let bot_name = bot.get_details().get_name().to_lowercase();
                if message.to_lowercase().contains(bot_name.as_str()) {
                    let unrecognised_speech = bot_data.get_unrecognised_speech();
                    if !unrecognised_speech.is_empty() {
                        let index =
                            rand::thread_rng().gen_range(0..unrecognised_speech.len());
                        let bot_speech = &unrecognised_speech[index];

                        bot.get_room_user()
                            .expect("bot must have a room user")
                            .talk(bot_speech.get_speech(), bot_speech.get_chat_message_type());
                    }
                }
            }
        }
    }

    /// Mirrors `HasRequestedDrink(Player, Bot, String)`.
    fn has_requested_drink(&self, player: &Player, bot: &Bot, message: &str) -> Option<String> {
        let bot_data = bot.get_bot_data().expect("bot must have data");

        if bot_data.get_drinks().is_empty() {
            return None;
        }

        let lowered_message = message.to_lowercase();

        for drink in bot_data.get_drinks() {
            if lowered_message.contains(drink.to_lowercase().as_str()) {
                player
                    .get_room_user()
                    .expect("player must have a room user")
                    .carry_item(-1, Some(drink.as_str()));
                return Some(drink.clone());
            }
        }

        if lowered_message.contains("drink please")
            || lowered_message.contains("can i have")
            || lowered_message.contains("i'll have")
        {
            let drinks = bot_data.get_drinks();
            let index = rand::thread_rng().gen_range(0..drinks.len());
            let drink = drinks[index].clone();

            player
                .get_room_user()
                .expect("player must have a room user")
                .carry_item(-1, Some(drink.as_str()));
            return Some(drink);
        }

        None
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static BotManager {
        INSTANCE.get_or_init(|| BotManager)
    }
}
