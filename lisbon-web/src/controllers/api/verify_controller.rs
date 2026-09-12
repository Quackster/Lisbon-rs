//! Mirrors `org.alexdev.http.controllers.api.VerifyController`.

use crate::dao::verify_dao::VerifyDao;
use crate::duckhttpd::WebConnection;

fn first_match(web_connection: &WebConnection) -> Option<String> {
    web_connection
        .get_matches()
        .first()
        .filter(|value| !value.trim().is_empty())
        .cloned()
}

/// Mirrors `get(WebConnection)`.
pub fn get(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let match_value = match first_match(web_connection) {
        Some(value) => value.to_string(),
        None => {
            web_connection.send_string("error: INVALID");
            return Ok(());
        }
    };

    match VerifyDao::get_name(&match_value) {
        Some(username) => web_connection.send_string(&username),
        None => web_connection.send_string("error: INVALID"),
    }

    Ok(())
}

/// Mirrors `clear(WebConnection)`.
pub fn clear(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let match_value = match first_match(web_connection) {
        Some(value) => value.to_string(),
        None => {
            web_connection.send_string("error: INVALID");
            return Ok(());
        }
    };

    VerifyDao::clear_name(&match_value);
    web_connection.send_string("SUCCESS");

    Ok(())
}
