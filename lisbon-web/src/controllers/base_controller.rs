//! Mirrors `org.alexdev.http.controllers.BaseController`.

use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::WebConnection;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

/// Mirrors `handleRoute(WebConnection)`.
pub fn handle_route(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if web_connection.is_request_handled() {
        if GameConfiguration::get_instance().get_bool("maintenance")
            && !web_connection.get_route_request().starts_with("/api")
        {
            if !web_connection.get_route_request().starts_with("/maintenance")
                && !web_connection
                    .get_route_request()
                    .starts_with(&format!("/{HOUSEKEEPING_PATH}"))
            {
                web_connection.redirect("/maintenance");
                return Ok(());
            }
        }
    }

    if !web_connection.get_route_request().starts_with("/api") {
        let headers = web_connection.request().headers();

        if !headers.is_empty() {
            let host = headers.get("Host");

            if headers.contains("X-Forwarded-Proto") {
                let request = headers.get("X-Forwarded-Proto");

                if let (Some(host), Some(request)) = (host, request) {
                    if request.eq_ignore_ascii_case("http") {
                        let mut target_url = format!("https://{host}");
                        let request_uri = web_connection.request().uri();

                        if !request_uri.starts_with('/') {
                            target_url.push('/');
                        }

                        target_url.push_str(&request_uri);

                        web_connection.moved_permanently(&target_url);
                        return Ok(());
                    }
                }
            }
        }
    }

    if web_connection.is_request_handled() {
        if web_connection.session().get_boolean("authenticated") {
            handle_authenticated_route(web_connection);
        } else {
            SessionUtil::check_cookie(web_connection);
        }
    }

    Ok(())
}

fn handle_authenticated_route(web_connection: &WebConnection) {
    if web_connection.get_route_request() == "/client" {
        web_connection.session().set(
            "lastRequest",
            SessionValue::Str(
                (DateUtil::get_current_time_seconds() as i64 + SessionUtil::REAUTHENTICATE_TIME as i64)
                    .to_string(),
            ),
        );
    }

    if web_connection.session().contains("lastRequest") {
        let last_request = web_connection.session().get_long("lastRequest");

        if (DateUtil::get_current_time_seconds() as i64) > last_request {
            web_connection.session().set("clientAuthenticate", SessionValue::Bool(true));
        }
    } else {
        web_connection.session().set("clientAuthenticate", SessionValue::Bool(false));
    }
}
