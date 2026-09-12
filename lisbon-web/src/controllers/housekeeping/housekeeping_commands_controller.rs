//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingCommandsController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::moderation::actions::moderator_ban_user_action::ModeratorBanUserAction;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::WebConnection;
use crate::util::rcon_util::RconUtil;
use crate::util::session_util::SessionUtil;

/// Mirrors `ban(WebConnection)`.
pub fn ban(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    // If they are logged in, send them to the /me page
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.send_string("");
    }

    let Some(username) = web_connection.get().get_string("username") else {
        web_connection.send_string("User doesn't exist");
        return Ok(());
    };

    let Some(player_details) = PlayerDao::get_details_by_name(&username) else {
        web_connection.send_string("User doesn't exist");
        return Ok(());
    };

    RconUtil::send_command(
        RconHeader::DisconnectUser,
        HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
    );

    let banning_id = web_connection.session().get_int("user.id");
    let Some(banning_player_details) = PlayerDao::get_details(banning_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    // The Java moderation log call is commented out in the
    // source.
    web_connection.send_string(
        &ModeratorBanUserAction::ban(
            &banning_player_details,
            "Banned for breaking the HabboWay",
            "",
            player_details.get_name(),
            999_999_999,
            true,
            true,
        ),
    );
    Ok(())
}
