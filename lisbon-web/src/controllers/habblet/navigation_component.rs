//! Mirrors `org.alexdev.http.controllers.habblet.NavigationComponent`.

use crate::duckhttpd::WebConnection;

/// Mirrors `navigation(WebConnection)`.
pub fn navigation(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.send_string("");
    Ok(())
}
