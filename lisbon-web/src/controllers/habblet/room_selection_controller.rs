//! Mirrors `org.alexdev.http.controllers.habblet.RoomSelectionController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;

use crate::duckhttpd::WebConnection;

/// Mirrors `confirm(WebConnection)`.
pub fn confirm(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("habblet/roomselectionConfirm");
    template.render();
    Ok(())
}

/// Mirrors `create(WebConnection)`.
pub fn create(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated")
        || !web_connection.post().contains("roomType")
    {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if !player_details.can_select_room() {
        web_connection.send_string("");
        return Ok(());
    }

    if let Some(room_type) = web_connection.post().get_int("roomType") {
        if room_type < 0 || room_type > 5 {
            web_connection.send_string("");
            return Ok(());
        }
    }

    web_connection.send_string("");
    Ok(())
}

/// Mirrors `hide(WebConnection)`.
pub fn hide(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let mut player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if !player_details.can_select_room() {
        web_connection.send_string("");
        return Ok(());
    }

    player_details.set_selected_room_id(-1);

    PlayerDao::save_selected_room(player_details.get_id(), -1);
    PlayerStatisticsDao::update_statistic(
        player_details.get_id(),
        PlayerStatistic::NewbieRoomLayout,
        "-1",
    );

    web_connection.send_string("");
    Ok(())
}
