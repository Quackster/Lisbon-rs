//! Mirrors `org.alexdev.http.controllers.api.ImagerController`.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Once;

use http::StatusCode;

use crate::duckhttpd::{ResponseBuilder, WebConnection};

static LOADED: Once = Once::new();

/// Mirrors `imagerRedirect(WebConnection)`.
pub fn imager_redirect(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    load_avatara();

    let request_uri = web_connection.request().uri();
    let path = request_uri.split('?').next().unwrap_or_default();

    if path.starts_with("/habbo-imaging/avatar/") {
        send_avatar(web_connection, legacy_avatar_parameters(path));
        return Ok(());
    }

    if path == "/habbo-imaging/avatarimage" {
        send_avatar(web_connection, parse_query(&request_uri));
        return Ok(());
    }

    if path.starts_with("/habbo-imaging/badge/") {
        send_badge(web_connection, path);
        return Ok(());
    }

    send_no_content(web_connection);
    Ok(())
}

fn load_avatara() {
    LOADED.call_once(|| {
        // The avatara4j `FiguredataReader`/`LegacyFiguredataReader`/`ManifestReader`
        // sprite data ships inside the external `avatara4j` jar, which has no Rust
        // equivalent, so there is nothing to preload.
    });
}

fn send_avatar(web_connection: &WebConnection, parameters: HashMap<String, String>) {
    let figure = parameters.get("figure").map(String::as_str).unwrap_or_default();

    if figure.trim().is_empty() {
        send_no_content(web_connection);
        return;
    }

    // Avatar rendering is delegated to the avatara4j external library, which has
    // no Rust equivalent; the `size`/`direction`/`head_direction`/`action`/
    // `gesture`/`headonly`/`frame`/`crr`/`crop` parameters are therefore dropped
    // and the request degrades to a no-content response.
    send_no_content(web_connection);
}

fn send_badge(web_connection: &WebConnection, path: &str) {
    let mut badge_code = path.trim_start_matches("/habbo-imaging/badge/").to_string();

    if badge_code.ends_with(".gif") {
        // The `RenderType.GIF` / gif content type are dropped until the avatara4j
        // library (external, not ported) has a Rust equivalent.
        badge_code.truncate(badge_code.len() - 4);
    } else if badge_code.ends_with(".png") {
        badge_code.truncate(badge_code.len() - 4);
    }

    if badge_code.trim().is_empty() {
        send_no_content(web_connection);
        return;
    }

    // Badge rendering is delegated to the avatara4j external library, which has
    // no Rust equivalent, so the request degrades to a no-content response.
    send_no_content(web_connection);
}

fn legacy_avatar_parameters(path: &str) -> HashMap<String, String> {
    let mut value = path.trim_start_matches("/habbo-imaging/avatar/").to_string();

    if value.contains(',') {
        value = value.split(',').next().unwrap_or_default().to_string();
    }

    let mut parameters = HashMap::new();
    parameters.insert("figure".to_string(), value);
    parameters.insert("size".to_string(), "b".to_string());
    parameters.insert("direction".to_string(), "3".to_string());
    parameters.insert("head_direction".to_string(), "3".to_string());
    parameters.insert("gesture".to_string(), "sml".to_string());
    parameters.insert("frame".to_string(), "1".to_string());
    parameters
}

fn parse_query(request_uri: &str) -> HashMap<String, String> {
    let mut parameters = HashMap::new();
    let parts: Vec<&str> = request_uri.splitn(2, '?').collect();

    if parts.len() < 2 {
        return parameters;
    }

    for pair in parts[1].split('&') {
        let key_value: Vec<&str> = pair.splitn(2, '=').collect();
        let key = decode(key_value[0]);
        let value = if key_value.len() > 1 {
            decode(key_value[1])
        } else {
            String::new()
        };
        parameters.insert(key, value);
    }

    parameters
}

fn decode(value: &str) -> String {
    urlencoding::decode(value).map(Cow::into_owned).unwrap_or_else(|_| value.to_string())
}

fn send_no_content(web_connection: &WebConnection) {
    web_connection
        .send(ResponseBuilder::create_with_status(StatusCode::NO_CONTENT, ""));
}
