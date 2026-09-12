//! Mirrors `org.alexdev.http.util.GroupUtil`.

use std::collections::HashMap;

use http::StatusCode;

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::game::groups::group::Group;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::Settings;
use crate::duckhttpd::web_connection::WebConnection;
use crate::util::rcon_util::RconUtil;

/// Mirrors `org.alexdev.http.util.GroupUtil`.
pub struct GroupUtil;

impl GroupUtil {
    /// Mirrors `refreshGroup(int)`.
    pub fn refresh_group(group_id: i32) {
        let mut parameters = HashMap::new();
        parameters.insert("groupId".to_string(), group_id.to_string());
        RconUtil::send_command(RconHeader::RefreshGroup, parameters);
    }

    /// Mirrors `resolve(WebConnection)`.
    pub fn resolve(web_connection: &WebConnection) -> Option<Group> {
        let match_ = web_connection.get_matches().first().cloned().unwrap_or_default();
        let mut group: Option<Group> = None;

        if !match_.is_empty()
            && match_.chars().all(|c| c.is_ascii_digit())
            && web_connection.get_route_request().ends_with("/id/discussions")
        {
            match GroupDao::get_group(match_.parse().unwrap_or(0)) {
                None => {
                    if let Some(response) = Settings::get_instance()
                        .get_default_responses()
                        .get_response(StatusCode::NOT_FOUND, web_connection)
                    {
                        web_connection.send(response);
                    }
                    return None;
                }
                Some(found) => {
                    if !found.get_alias().trim().is_empty() {
                        web_connection.redirect(&format!(
                            "/groups/{}/discussions",
                            found.get_alias()
                        ));
                        return None;
                    }
                    group = Some(found);
                }
            }
        } else if !web_connection.get_route_request().ends_with("/id/discussions") {
            group = GroupDao::get_group_by_alias(&match_);
        }

        group
    }
}
