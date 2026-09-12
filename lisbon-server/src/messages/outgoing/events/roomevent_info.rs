//! Mirrors `net.h4bbo.lisbon.messages.outgoing.events.ROOMEEVENT_INFO`.
use crate::game::events::event::Event;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOMEEVENT_INFO {
    event: Option<Event>,
}

impl ROOMEEVENT_INFO {
    /// Mirrors the `ROOMEEVENT_INFO(Event)` constructor.
    pub fn new(event: Option<Event>) -> Self {
        Self { event }
    }
}

impl MessageComposer for ROOMEEVENT_INFO {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        match &self.event {
            None => {
                response.write_string(-1);
            }
            Some(event) => {
                if let Some(event_hoster) = event.get_event_hoster() {
                    response.write_string(event_hoster.get_id());
                    response.write_string(event_hoster.get_name());
                }
                response.write_string(event.get_room_id());
                response.write_int(event.get_category_id());
                response.write_string(event.get_name());
                response.write_string(event.get_description());
                response.write_string(event.get_started_date().unwrap_or_default());
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        370
    }
}
