//! Mirrors `org.alexdev.http.controllers.habblet.NameCheckController`.

use crate::duckhttpd::{ResponseBuilder, WebConnection};
use crate::util::register_util::RegisterUtil;

/// Mirrors `nameCheck(WebConnection)`.
pub fn namecheck(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut error_message = "";

    let username = web_connection.post().get_string("name").unwrap_or_default();
    let error_code = RegisterUtil::get_name_error_code(&username);

    match error_code {
        6 => error_message = "This name is unacceptable to hotel management.",
        5 => error_message = "Your username is invalid or contains invalid characters.",
        4 => error_message = "This name is not allowed.",
        3 => error_message = "The name you have chosen is too long.",
        2 => error_message = "Please enter a username.",
        1 => error_message = "A user with this name already exists.",
        _ => {}
    }

    let mut response = ResponseBuilder::create("");
    response.headers().push((
        "X-JSON".to_string(),
        format!("{{\"registration_name\":\"{error_message}\"}}"),
    ));
    web_connection.send(response);
    Ok(())
}

/// Mirrors `hasAllowedCharacters(String, String)`.
pub fn has_allowed_characters(str: Option<&str>, allowed_chars: &str) -> bool {
    let Some(str) = str else {
        return false;
    };

    for character in str.chars() {
        if allowed_chars.contains(character) {
            continue;
        }

        return false;
    }

    true
}
