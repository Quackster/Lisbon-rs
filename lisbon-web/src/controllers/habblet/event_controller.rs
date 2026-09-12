//! Mirrors `org.alexdev.http.controllers.habblet.EventController`.

use lisbon_server::game::events::event::Event;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::server::watchdog::EVENTS;

/// Mirrors `loadEvents(WebConnection)`.
pub fn load_events(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.post().contains("eventTypeId") {
        web_connection.send_string("");
        return Ok(());
    }

    let filter_id = match web_connection.post().get_int("eventTypeId") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    let mut template = web_connection.template("habblet/load_events");
    let events: Vec<Event> = EVENTS
        .read()
        .iter()
        .filter(|event| event.get_category_id() == filter_id)
        .cloned()
        .collect();
    template.set("events", TemplateValue::of(events));
    template.render();
    Ok(())
}
