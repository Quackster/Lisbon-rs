//! Mirrors `net.h4bbo.lisbon.messages.outgoing.events.ROOMEVENT_LIST`.
use crate::game::events::event::Event;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ROOMEVENT_LIST {
    type_id: i32,
    events: Vec<Event>,
}

impl ROOMEVENT_LIST {
    /// Mirrors the `ROOMEVENT_LIST(int, List<Event>)` constructor.
    pub fn new(type_id: i32, events: Vec<Event>) -> Self {
        Self { type_id, events }
    }
}

impl MessageComposer for ROOMEVENT_LIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.type_id);
        response.write_int(self.events.len() as i32);

        for event in &self.events {
            response.write_string(event.get_room_id());
            response.write_string(
                event
                    .get_event_hoster()
                    .map(|hoster| hoster.get_name().to_string())
                    .unwrap_or_default(),
            );
            response.write_string(event.get_name());
            response.write_string(event.get_description());
            response.write_string(event.get_started_date().unwrap_or_default());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        369 // "Eq"
    }
}
