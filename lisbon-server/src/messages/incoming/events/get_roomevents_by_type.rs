//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.GET_ROOMEVENTS_BY_TYPE`.
use crate::game::events::events_manager::EventsManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_list::ROOMEVENT_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_ROOMEVENTS_BY_TYPE;

impl MessageEvent for GET_ROOMEVENTS_BY_TYPE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let category_id = reader.read_int();

        if category_id < 0 || category_id > 11 {
            return Ok(());
        }

        let event_list = EventsManager::get_instance().get_events(category_id);

        player.send(&ROOMEVENT_LIST::new(category_id, event_list));

        Ok(())
    }
}
