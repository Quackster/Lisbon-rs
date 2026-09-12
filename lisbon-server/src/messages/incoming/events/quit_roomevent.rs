//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.QUIT_ROOMEVENT`.
use crate::game::events::events_manager::EventsManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_info::ROOMEEVENT_INFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct QUIT_ROOMEVENT;

impl MessageEvent for QUIT_ROOMEVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id()) {
            return Ok(());
        }

        let events_manager = EventsManager::get_instance();

        if !events_manager.has_event(room.get_id()) {
            return Ok(());
        }

        let Some(event) = events_manager.get_event_by_room_id(room.get_id()) else {
            return Ok(());
        };

        events_manager.remove_event(&event);
        room.send(&ROOMEEVENT_INFO::new(None));

        Ok(())
    }
}
