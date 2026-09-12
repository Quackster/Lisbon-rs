//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.CAN_CREATE_ROOMEVENT`.
use crate::game::events::events_manager::EventsManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_permission::ROOMEVENT_PERMISSION;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CAN_CREATE_ROOMEVENT;

impl MessageEvent for CAN_CREATE_ROOMEVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let can_create_event = EventsManager::get_instance().can_create_event(player);

        player.send(&ROOMEVENT_PERMISSION::new(can_create_event));

        Ok(())
    }
}
