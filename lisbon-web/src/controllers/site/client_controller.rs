//! Mirrors `org.alexdev.http.controllers.site.ClientController`.

use std::sync::atomic::Ordering;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::room::handlers::room_selection_handler::RoomSelectionHandler;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::controllers::site::random_uuid;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::ResponseBuilder;
use crate::duckhttpd::web_connection::SessionValue;
use crate::server::watchdog::USERS_ONLNE;
use crate::util::session_util::SessionUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

fn is_numeric(value: Option<&str>) -> bool {
    value.map_or(false, |value| !value.is_empty() && value.chars().all(|character| character.is_ascii_digit()))
}

/// Mirrors `NumberFormat.getNumberInstance(Locale.US).format(int)` digit
/// grouping (e.g. `1234` -> `1,234`).
fn group_digits(value: i32) -> String {
    let negative = value < 0;
    let digits: String = value.unsigned_abs().to_string();
    let len = digits.len();
    let mut grouped = String::new();

    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (len - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(character);
    }

    if negative {
        grouped.insert(0, '-');
    }

    grouped
}

/// Mirrors `client(WebConnection)`.
pub fn client(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/login_popup");
        return Ok(());
    }

    let uri = web_connection.request().uri();
    let get_requests = if uri.contains('?') {
        format!("?{}", uri.split('?').nth(1).unwrap_or_default())
    } else {
        String::new()
    };

    web_connection.redirect(&format!("/shockwave_client{get_requests}"));

    Ok(())
}

/// Mirrors `shockwaveclient(WebConnection)`.
pub fn shockwaveclient(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/login_popup");
        return Ok(());
    }

    web_connection
        .session()
        .set("clientRequest", SessionValue::Str(web_connection.request().uri()));

    if web_connection.session().get_boolean("clientAuthenticate") {
        web_connection.redirect("/account/reauthenticate");
        return Ok(());
    }

    let mut forward_room = false;
    let mut forward_type = -1;
    let mut forward_id = -1;

    let mut template = web_connection.template("client");
    let user_id = web_connection.session().get_int("user.id");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            SessionUtil::logout(web_connection);
            web_connection.redirect("/");
            return Ok(());
        }
    };

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    if web_connection.get().contains("createRoom")
        && is_numeric(web_connection.get().get_string("createRoom").as_deref())
    {
        let room_type = web_connection
            .get()
            .get_string("createRoom")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let mut set_gift = false;

        if !player_details.can_select_room() {
            let room_layout = PlayerStatisticsDao::get_statistic_long(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
            );

            if room_layout == 0 {
                if !(room_type < 0 || room_type > 5) {
                    set_gift = true;
                }
            }
        } else {
            set_gift = RoomSelectionHandler::select_room(player_details.get_id(), room_type);
        }

        if set_gift {
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
                &(room_type + 1).to_string(),
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGift,
                "1",
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGiftTime,
                &(DateUtil::get_current_time_seconds() as i64 + 86400).to_string(),
            );
        }

        let refreshed = PlayerDao::get_details(user_id);
        forward_room = true;

        forward_type = 2; // Private room
        forward_id = refreshed.map_or(-1, |details| details.get_selected_room_id());
    }

    if web_connection.get().contains("forwardId") {
        forward_room = true;
        forward_id = web_connection
            .get()
            .get_int("roomId")
            .unwrap_or(forward_id);
        forward_type = web_connection
            .get()
            .get_int("forwardId")
            .unwrap_or(forward_type);
    }

    if web_connection.get().contains("shortcut") {
        let mut redirection_id = 0;

        if web_connection.get().get_string("shortcut").as_deref() == Some("roomomatic") {
            redirection_id = 1;
        }

        if redirection_id > 0 {
            template.set(
                "shortcut",
                TemplateValue::of(format!("shortcut.id={redirection_id};")),
            );
        }
    }

    let mut sso_ticket = player_details.get_sso_ticket().to_string();

    // Update sso ticket
    if GameConfiguration::get_instance().get_bool("reset.sso.after.login")
        || sso_ticket.trim().is_empty()
    {
        sso_ticket = random_uuid();
        PlayerDao::set_ticket(user_id, &sso_ticket);
    }

    template.set("ssoTicket", TemplateValue::of(sso_ticket));
    template.set("forwardRoom", TemplateValue::of(forward_room));

    if forward_room {
        template.set(
            "forward",
            TemplateValue::of(format!(
                "<param name=\"sw9\" value=\"forward.type={forward_type};forward.id={forward_id};processlog.url=\">"
            )),
        );
        template.set(
            "forwardSub",
            TemplateValue::of(format!(
                "sw9=\"forward.type={forward_type};forward.id={forward_id};processlog.url=\""
            )),
        );
        template.set(
            "forwardScript",
            TemplateValue::of(format!(
                "<param name=\\\"sw9\\\" value=\\\"forward.type={forward_type};forward.id={forward_id};processlog.url=\\\">",
            )),
        );
        template.set(
            "forwardSubScript",
            TemplateValue::of(format!(
                "sw9=\\\"forward.type={forward_type};forward.id={forward_id};processlog.url=\\\"",
            )),
        );
    }

    template.render();

    Ok(())
}

/// Mirrors `flashClient(WebConnection)`.
pub fn flash_client(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/login_popup");
        return Ok(());
    }

    let mut template = web_connection.template("client_flash");
    let user_id = web_connection.session().get_int("user.id");

    let player_details = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()));

    web_connection
        .session()
        .set("clientRequest", SessionValue::Str(web_connection.request().uri()));

    if web_connection.session().get_boolean("clientAuthenticate") {
        web_connection.redirect("/account/reauthenticate");
        return Ok(());
    }

    let player_details = match player_details {
        Some(details) => details,
        None => {
            SessionUtil::logout(web_connection);
            web_connection.redirect("/");
            return Ok(());
        }
    };

    let mut forward_room = false;
    let mut forward_type = -1;
    let mut forward_id = -1;

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    if web_connection.get().contains("createRoom")
        && is_numeric(web_connection.get().get_string("createRoom").as_deref())
    {
        let room_type = web_connection
            .get()
            .get_string("createRoom")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let mut set_gift = false;

        if !player_details.can_select_room() {
            let room_layout = PlayerStatisticsDao::get_statistic_long(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
            );

            if room_layout == 0 {
                if !(room_type < 0 || room_type > 5) {
                    set_gift = true;
                }
            }
        } else {
            set_gift = RoomSelectionHandler::select_room(player_details.get_id(), room_type);
        }

        if set_gift {
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
                &(room_type + 1).to_string(),
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGift,
                "1",
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGiftTime,
                &(DateUtil::get_current_time_seconds() as i64 + 86400).to_string(),
            );
        }

        let refreshed = PlayerDao::get_details(user_id);
        forward_room = true;

        forward_type = 2; // Private room
        forward_id = refreshed.map_or(-1, |details| details.get_selected_room_id());
    }

    if web_connection.get().contains("forwardId") {
        forward_room = true;
        forward_id = web_connection
            .get()
            .get_int("roomId")
            .unwrap_or(forward_id);
        forward_type = web_connection
            .get()
            .get_int("forwardId")
            .unwrap_or(forward_type);
    }

    template.set("forwardRoom", TemplateValue::of(forward_room));
    template.set("forwardId", TemplateValue::of(forward_id));
    template.set("forwardType", TemplateValue::of(forward_type));

    // Update sso ticket
    let mut sso_ticket = player_details.get_sso_ticket().to_string();

    if GameConfiguration::get_instance().get_bool("reset.sso.after.login") || sso_ticket.trim().is_empty()
    {
        sso_ticket = random_uuid();
        PlayerDao::set_ticket(user_id, &sso_ticket);
    }

    template.set("ssoTicket", TemplateValue::of(sso_ticket));
    template.render();

    Ok(())
}

/// Mirrors `clientInstallShockwave(WebConnection)`.
pub fn client_install_shockwave(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/login_popup");
        return Ok(());
    }

    let mut template = web_connection.template("client_install_shockwave");

    if let Some(player_details) = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        if player_details.is_banned().is_some() {
            web_connection.redirect("/account/banned");
            return Ok(());
        }
    }

    template.render();

    Ok(())
}

/// Mirrors `updateHabboCount(WebConnection)`.
pub fn update_habbo_count(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = ResponseBuilder::create("");
    let users_online = USERS_ONLNE.load(Ordering::Relaxed);
    let grouped = group_digits(users_online);
    response.set_header(
        "X-JSON",
        &format!("{{\"habboCountText\":\"{grouped} members online\"}}"),
    );
    web_connection.send(response);

    Ok(())
}

/// Mirrors `blank(WebConnection)`.
pub fn blank(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.send_string("");

    Ok(())
}

/// Mirrors `client_error(WebConnection)`.
pub fn client_error(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("client_error");

    if web_connection.session().get_boolean("authenticated") {
        if let Some(player_details) = template
            .get("playerDetails")
            .and_then(|value| PlayerDetails::from_json(value.value()))
        {
            if player_details.is_banned().is_some() {
                web_connection.redirect("/account/banned");
                return Ok(());
            }
        }
    }

    if web_connection.get().contains("error_id") {
        template.set(
            "errorId",
            TemplateValue::of(
                web_connection
                    .get()
                    .get_string("error_id")
                    .unwrap_or_default(),
            ),
        );
    }

    template.render();

    Ok(())
}

/// Mirrors `client_connection_failed(WebConnection)`.
pub fn client_connection_failed(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("client_connection_failed");

    if web_connection.session().get_boolean("authenticated") {
        if let Some(player_details) = template
            .get("playerDetails")
            .and_then(|value| PlayerDetails::from_json(value.value()))
        {
            if player_details.is_banned().is_some() {
                web_connection.redirect("/account/banned");
                return Ok(());
            }
        }
    }

    if web_connection.get().contains("error_id") {
        template.set(
            "errorId",
            TemplateValue::of(
                web_connection
                    .get()
                    .get_string("error_id")
                    .unwrap_or_default(),
            ),
        );
    }

    template.render();

    Ok(())
}

/// Mirrors `betaClient(WebConnection)`.
pub fn beta_client(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/login_popup");
        return Ok(());
    }

    let mut template = web_connection.template("client_beta");
    let user_id = web_connection.session().get_int("user.id");

    let player_details = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()));

    web_connection
        .session()
        .set("clientRequest", SessionValue::Str(web_connection.request().uri()));

    if web_connection.session().get_boolean("clientAuthenticate") {
        web_connection.redirect("/account/reauthenticate");
        return Ok(());
    }

    let player_details = match player_details {
        Some(details) => details,
        None => {
            SessionUtil::logout(web_connection);
            web_connection.redirect("/");
            return Ok(());
        }
    };

    let mut forward_room = false;
    let mut forward_type = -1;
    let mut forward_id = -1;

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    if web_connection.get().contains("createRoom")
        && is_numeric(web_connection.get().get_string("createRoom").as_deref())
    {
        let room_type = web_connection
            .get()
            .get_string("createRoom")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let mut set_gift = false;

        if !player_details.can_select_room() {
            let room_layout = PlayerStatisticsDao::get_statistic_long(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
            );

            if room_layout == 0 {
                if !(room_type < 0 || room_type > 5) {
                    set_gift = true;
                }
            }
        } else {
            set_gift = RoomSelectionHandler::select_room(player_details.get_id(), room_type);
        }

        if set_gift {
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieRoomLayout,
                &(room_type + 1).to_string(),
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGift,
                "1",
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::NewbieGiftTime,
                &(DateUtil::get_current_time_seconds() as i64 + 86400).to_string(),
            );
        }

        let refreshed = PlayerDao::get_details(user_id);
        forward_room = true;

        forward_type = 2; // Private room
        forward_id = refreshed.map_or(-1, |details| details.get_selected_room_id());
    }

    if web_connection.get().contains("forwardId") {
        forward_room = true;
        forward_id = web_connection
            .get()
            .get_int("roomId")
            .unwrap_or(forward_id);
        forward_type = web_connection
            .get()
            .get_int("forwardId")
            .unwrap_or(forward_type);
    }

    template.set("forwardRoom", TemplateValue::of(forward_room));
    template.set("forwardId", TemplateValue::of(forward_id));
    template.set("forwardType", TemplateValue::of(forward_type));

    // Update sso ticket
    let mut sso_ticket = player_details.get_sso_ticket().to_string();

    if GameConfiguration::get_instance().get_bool("reset.sso.after.login")
        || sso_ticket.trim().is_empty()
    {
        sso_ticket = random_uuid();
        PlayerDao::set_ticket(user_id, &sso_ticket);
    }

    template.set("ssoTicket", TemplateValue::of(sso_ticket));
    template.render();

    Ok(())
}
