//! Mirrors `net.h4bbo.lisbon.game.bot.Bot`.
//!
//! Java `Bot extends Entity`; the `Entity` class hierarchy is mirrored by the
//! `Entity` trait.

use crate::game::bot::bot_data::BotData;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_details::PlayerDetails;
use crate::game::room::entities::room_bot::RoomBot;
use crate::game::room::entities::room_entity::RoomEntity;

#[derive(Clone)]
pub struct Bot {
    player_details: PlayerDetails,
    // Port note: the Java field is `RoomBot roomUser` (a `RoomEntity`
    // subclass); Rust has no subtyping, so the base entity is composed.
    room_user: RoomBot,
    bot_data: Option<BotData>,
    next_walk_time: i64,
    next_speech_time: i64,
}

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}

impl Bot {
    /// Mirrors the no-arg `Bot()` constructor.
    pub fn new() -> Self {
        Self {
            player_details: PlayerDetails::new(),
            // Port note: the Java constructor casts `this` to `Entity`.
            room_user: RoomBot::new(),
            bot_data: None,
            next_walk_time: 0,
            next_speech_time: 0,
        }
    }

    /// Mirrors the `Bot(BotData)` constructor.
    pub fn with_data(bot_data: BotData) -> Self {
        let mut bot = Self::new();
        bot.bot_data = Some(bot_data);
        bot
    }

    /// Mirrors `getDetails` (mutable; the Java 5-arg `PlayerDetails.fill`
    /// overload needs `&mut self`).
    pub fn get_details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.player_details
    }

    /// Mirrors `getBotData`.
    pub fn get_bot_data(&self) -> Option<&BotData> {
        self.bot_data.as_ref()
    }

    /// Mirrors `getNextWalkTime`.
    pub fn get_next_walk_time(&self) -> i64 {
        self.next_walk_time
    }

    /// Mirrors `setNextWalkTime`.
    pub fn set_next_walk_time(&mut self, next_walk_time: i64) {
        self.next_walk_time = next_walk_time
    }

    /// Mirrors `getNextSpeechTime`.
    pub fn get_next_speech_time(&self) -> i64 {
        self.next_speech_time
    }

    /// Mirrors `setNextSpeechTime`.
    pub fn set_next_speech_time(&mut self, next_speech_time: i64) {
        self.next_speech_time = next_speech_time
    }
}

impl Entity for Bot {
    /// Mirrors `hasFuse(Fuseright)`.
    fn has_fuse(&self, _permission: &Fuseright) -> bool {
        false
    }

    /// Mirrors `getDetails`.
    fn get_details(&self) -> &PlayerDetails {
        &self.player_details
    }

    /// Mirrors `getRoomUser`.
    fn get_room_user(&self) -> Option<&RoomEntity> {
        Some(&self.room_user.entity)
    }

    /// Mirrors `getType`.
    fn get_type(&self) -> EntityType {
        EntityType::Bot
    }

    /// Mirrors `dispose`.
    fn dispose(&mut self) {}

    /// Mirrors the `instanceof Bot` cast.
    fn as_bot(&self) -> Option<&crate::game::bot::bot::Bot> {
        Some(self)
    }
}
