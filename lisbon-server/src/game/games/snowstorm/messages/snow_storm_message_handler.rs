//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.messages.SnowStormMessageHandler`.
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::messages::incoming::snow_storm_attack_player_message::SnowStormAttackPlayerMessage;
use crate::game::games::snowstorm::messages::incoming::snow_storm_throw_location_message::SnowStormThrowLocationMessage;
use crate::game::games::snowstorm::messages::incoming::snow_storm_walk_message::SnowStormWalkMessage;
use crate::game::games::snowstorm::messages::incoming::snowstorm_create_snowball_message::SnowstormCreateSnowballMessage;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::util::snow_storm_event::SnowStormEvent;
use crate::game::games::snowstorm::util::snow_storm_message::SnowStormMessage;
use crate::server::netty::streams::NettyRequest;
use parking_lot::Mutex;

pub struct SnowStormMessageHandler {
    events: HashMap<SnowStormEvent, Box<dyn SnowStormMessage>>,
}

impl SnowStormMessageHandler {
    /// Mirrors the `SnowStormMessageHandler` constructor.
    pub fn new() -> Self {
        let mut events = HashMap::new();

        events.insert(
            SnowStormEvent::Walk,
            Box::new(SnowStormWalkMessage) as Box<dyn SnowStormMessage>,
        );
        events.insert(
            SnowStormEvent::CreateSnowball,
            Box::new(SnowstormCreateSnowballMessage) as Box<dyn SnowStormMessage>,
        );
        events.insert(
            SnowStormEvent::ThrowSnowballAtLocation,
            Box::new(SnowStormThrowLocationMessage) as Box<dyn SnowStormMessage>,
        );
        events.insert(
            SnowStormEvent::ThrowSnowballAtPerson,
            Box::new(SnowStormAttackPlayerMessage) as Box<dyn SnowStormMessage>,
        );

        Self { events }
    }

    /// Mirrors `handleMessage(int, NettyRequest, SnowStormGame, GamePlayer)`.
    pub fn handle_message(
        &self,
        message_id: i32,
        request: &mut NettyRequest,
        snow_storm_game: &Arc<SnowStormGame>,
        player: &Arc<Mutex<GamePlayer>>,
    ) {
        let event = SnowStormEvent::get_event(message_id);

        if let Some(event) = event {
            if let Some(message) = self.events.get(&event) {
                message.handle(request, snow_storm_game, player);
            }
        }
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<SnowStormMessageHandler>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::new())).clone()
    }
}
