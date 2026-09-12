//! Mirrors `net.h4bbo.lisbon.game.bot.BotData`.

use crate::game::bot::bot_speech::BotSpeech;
use crate::game::pathfinder::position::Position;
use crate::util::string_util::StringUtil;

#[derive(Clone)]
pub struct BotData {
    name: String,
    mission: String,
    start_position: Position,
    figure: String,
    walkspace: Vec<Position>,
    speeches: Vec<BotSpeech>,
    responses: Vec<BotSpeech>,
    unrecognised_speech: Vec<BotSpeech>,
    drinks: Vec<String>,
}

impl BotData {
    /// Mirrors the 12-arg `BotData(String, String, int, int, int, int, String,
    /// String, String, String, String, String)` constructor.
    pub fn new(
        name: &str,
        mission: &str,
        x: i32,
        y: i32,
        head_rotation: i32,
        body_rotation: i32,
        figure: &str,
        walkspace_data: &str,
        speech: &str,
        responses: &str,
        unrecognised_responses: &str,
        drinks: &str,
    ) -> Self {
        let start_position = Position::with_rotations(x, y, 0.0, head_rotation, body_rotation);

        let mut walkspace: Vec<Position> = Vec::new();
        for position_data in walkspace_data.split(' ') {
            let coords: Vec<&str> = position_data.split(',').collect();
            if coords.len() >= 2 {
                let wx: i32 = coords[0].parse().unwrap_or(0);
                let wy: i32 = coords[1].parse().unwrap_or(0);
                walkspace.push(Position::new_xy(wx, wy));
            }
        }

        if !walkspace.iter().any(|position| {
            position.get_x() == start_position.get_x() && position.get_y() == start_position.get_y()
        }) {
            walkspace.push(Position::new_xy(
                start_position.get_x(),
                start_position.get_y(),
            ));
        }

        Self {
            name: name.to_string(),
            mission: mission.to_string(),
            start_position,
            figure: figure.to_string(),
            walkspace,
            speeches: Self::parse_speech(speech),
            responses: Self::parse_speech(responses),
            unrecognised_speech: Self::parse_speech(unrecognised_responses),
            drinks: if drinks.is_empty() {
                Vec::new()
            } else {
                drinks.split(',').map(String::from).collect()
            },
        }
    }

    fn parse_speech(responses: &str) -> Vec<BotSpeech> {
        let mut bot_speech: Vec<BotSpeech> = Vec::new();

        for sentence in responses.split('|') {
            let text = StringUtil::filter_input(sentence, true);

            if text.is_empty() {
                continue;
            }

            bot_speech.push(BotSpeech::new(sentence));
        }

        bot_speech
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getMission`.
    pub fn get_mission(&self) -> &str {
        &self.mission
    }

    /// Mirrors `getStartPosition`.
    pub fn get_start_position(&self) -> &Position {
        &self.start_position
    }

    /// Mirrors `getFigure`.
    pub fn get_figure(&self) -> &str {
        &self.figure
    }

    /// Mirrors `getWalkspace`.
    pub fn get_walkspace(&self) -> &[Position] {
        &self.walkspace
    }

    /// Mirrors `getSpeeches`.
    pub fn get_speeches(&self) -> &[BotSpeech] {
        &self.speeches
    }

    /// Mirrors `getResponses`.
    pub fn get_responses(&self) -> &[BotSpeech] {
        &self.responses
    }

    /// Mirrors `getUnrecognisedSpeech`.
    pub fn get_unrecognised_speech(&self) -> &[BotSpeech] {
        &self.unrecognised_speech
    }

    /// Mirrors `getDrinks`.
    pub fn get_drinks(&self) -> &[String] {
        &self.drinks
    }
}
