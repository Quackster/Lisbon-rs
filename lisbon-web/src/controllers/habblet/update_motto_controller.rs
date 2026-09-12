//! Mirrors `org.alexdev.http.controllers.habblet.UpdateMottoController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{ResponseBuilder, WebConnection};
use crate::util::html_util::HtmlUtil;
use crate::util::rcon_util::RconUtil;

/// Mirrors `updateMotto(WebConnection)`.
pub fn updatemotto(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if web_connection.session().contains("lastMottoUpdate") {
        let last_update_date = web_connection.session().get_long_or(
            "lastMottoUpdate",
            DateUtil::get_current_time_seconds() as i64,
        );

        if (DateUtil::get_current_time_seconds() as i64) < last_update_date {
            let response = ResponseBuilder::create(format!(
                "<script>document.getElementById(\"habbo-plate\").innerHTML = \"<img src='{}' style='margin-top: 57px'>\";</script>{}",
                GameConfiguration::get_instance().get_string("site.path"),
                HtmlUtil::escape(player_details.get_motto())
            ));
            web_connection.send(response);
            return Ok(());
        }
    }

    web_connection.session().set(
        "lastMottoUpdate",
        SessionValue::Str(DateUtil::get_current_time_seconds().to_string()),
    );

    let mut response_motto = String::new();
    let mut motto = WordfilterManager::filter_sentence(&HtmlUtil::remove_html_tags(
        &web_connection.post().get_string("motto").unwrap_or_default(),
    ));

    if motto.chars().count() > 100 {
        motto = motto.chars().take(100).collect();
    }

    if motto.replace(' ', "").is_empty() {
        response_motto = "Click to enter your motto/ status".to_string();
        motto = String::new();
    } else if motto.to_lowercase() == "crikey" {
        response_motto = format!(
            "<script>document.getElementById(\"habbo-plate\").innerHTML = \"<img src='{}' style='margin-top: 57px'>\";</script>",
            GameConfiguration::get_instance().get_string("site.path")
        );
    }

    if player_details.get_motto() != motto {
        PlayerDao::save_motto(user_id, &motto);

        if player_details.is_online() {
            RconUtil::send_command(
                RconHeader::RefreshLooks,
                HashMap::from([("userId".to_string(), user_id.to_string())]),
            );
        }
    }

    let response = ResponseBuilder::create(response_motto + &HtmlUtil::escape(&motto));
    web_connection.send(response);
    Ok(())
}
