//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.GET_ROOMEVENT_TYPE_COUNT`.
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_types::ROOMEVENT_TYPES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

#[allow(non_camel_case_types)]
pub struct GET_ROOMEVENT_TYPE_COUNT;

impl MessageEvent for GET_ROOMEVENT_TYPE_COUNT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&ROOMEVENT_TYPES::new(
            GameConfiguration::get_instance().get_integer("events.category.count"),
        ));

        Ok(())
    }
}
