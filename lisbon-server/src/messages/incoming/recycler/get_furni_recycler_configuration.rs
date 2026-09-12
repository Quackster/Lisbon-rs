//! Mirrors `net.h4bbo.lisbon.messages.incoming.recycler.GET_FURNI_RECYCLER_CONFIGURATION`.
use crate::game::player::player::Player;
use crate::game::recycler::recycler_manager::RecyclerManager;
use crate::messages::outgoing::recycler::recycler_configuration::RECYCLER_CONFIGURATION;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_FURNI_RECYCLER_CONFIGURATION;

impl MessageEvent for GET_FURNI_RECYCLER_CONFIGURATION {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let manager = RecyclerManager::get_instance();

        player.send(&RECYCLER_CONFIGURATION::new(
            manager.is_recycler_enabled(),
            manager.get_recycler_rewards(),
            manager.get_recycler_timeout_seconds(),
            manager.get_recycler_item_quarantine_seconds(),
            manager.get_recycler_session_length_seconds(),
        ));

        Ok(())
    }
}
