//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingUsersController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::housekeeping::housekeeping_player_dao::HousekeepingPlayerDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::util::email_util::EmailUtil;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

fn check_permission(
    web_connection: &WebConnection,
    permission: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(false);
    };

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        permission,
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    Ok(true)
}

/// Mirrors `imitate(WebConnection)`.
pub fn imitate(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let matches = web_connection.get_matches();
    let Some(player_name) = matches.first() else {
        // Java checks `playerName == null` after use.
        return Ok(());
    };

    let Some(player) = PlayerDao::get_details_by_name(player_name) else {
        return Ok(());
    };

    web_connection
        .session()
        .set("authenticated", SessionValue::Bool(true));
    web_connection
        .session()
        .set("captcha.invalid", SessionValue::Bool(false));
    web_connection
        .session()
        .set("user.id", SessionValue::Int(player.get_id()));
    web_connection
        .session()
        .set("clientAuthenticate", SessionValue::Bool(false));
    web_connection
        .session()
        .set(SessionUtil::LOGGED_IN_HOUSKEEPING, SessionValue::Bool(false));
    web_connection.session().set(
        "lastRequest",
        SessionValue::Str(
            (DateUtil::get_current_time_seconds() as i64 + SessionUtil::REAUTHENTICATE_TIME as i64)
                .to_string(),
        ),
    );
    web_connection.redirect("/me");
    Ok(())
}

/// Mirrors `search(WebConnection)`.
pub fn search(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "user/search")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/users_search");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().queries().len() > 0 {
        let field_check = ["searchField", "searchQuery", "searchType"];

        for field in field_check {
            if !(web_connection.post().contains(field)
                && web_connection
                    .post()
                    .get_string(field)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false))
            {
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("danger".into()));
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("You need to enter all fields".into()),
                );
                tpl.render_html().ok();

                // Delete alert after it's been rendered
                web_connection.session().delete("alertMessage");
                return Ok(());
            }
        }

        let field = web_connection.post().get_string("searchField").unwrap_or_default();
        let input = web_connection.post().get_string("searchQuery").unwrap_or_default();
        let search_type = web_connection.post().get_string("searchType").unwrap_or_default();

        let whitelist_columns = ["username", "id", "credits", "pixels", "mission"];

        let players = if whitelist_columns.contains(&field.as_str()) {
            HousekeepingPlayerDao::search(&search_type, &field, &input)
        } else {
            Vec::new()
        };

        tpl.set("players", TemplateValue::of(players));
    }

    tpl.set("pageName", TemplateValue::of("Search Users"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `create(WebConnection)`.
pub fn create(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "user/create")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/users_create");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    // The Java `defaultFigure` / `defaultMission` /
    // `defaultCredits` / `defaultDuckets` block is commented out in the
    // source.

    if web_connection.post().queries().len() > 0 {
        let field_check = [
            "username",
            "password",
            "confirmpassword",
            "figure",
            "email",
            "mission",
        ];

        for field in field_check {
            if !(web_connection.post().contains(field)
                && web_connection
                    .post()
                    .get_string(field)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false))
            {
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("danger".into()));
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("You need to enter all fields".into()),
                );
            }
        }

        if !web_connection.session().contains("alertMessage") {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("warning".into()));

            // The Java `emailExists` check is commented out in
            // the source.

            let password = web_connection.post().get_string("password").unwrap_or_default();
            let confirm_password =
                web_connection.post().get_string("confirmpassword").unwrap_or_default();
            let email = web_connection.post().get_string("email").unwrap_or_default();

            if password != confirm_password {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("The two passwords do not match".into()),
                );
            } else if password.chars().count() < 6 {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        "The password needs to be at least 6 or more characters".into(),
                    ),
                );
            } else if !EmailUtil::is_valid_email_address(&email) {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("The email entered is not valid".into()),
                );
            }
        }

        // Successful maybe?
        if web_connection.post().queries().len() > 0
            && !web_connection.session().contains("alertMessage")
        {
            let user_id: i32 = -1;
            // The Java `PlayerDao.create` call is commented out
            // in the source.

            web_connection
                .session()
                .set("alertColour", SessionValue::Str("success".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str(
                    format!(
                        "The new user has been successfully created. <a href=\"/houskeeping/users/edit?id={user_id}\">Click here</a> to edit them."
                    )
                    .into(),
                ),
            );
        }
    }

    tpl.set("pageName", TemplateValue::of("Create User"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `edit(WebConnection)`.
pub fn edit(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "user/edit")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/users_edit");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("You did not select a user to edit".into()),
        );
    }

    if web_connection.post().queries().len() > 0 {
        let field_check = ["username", "figure", "email", "motto", "credits", "pixels"];

        for field in field_check {
            if !(web_connection.post().contains(field)
                && web_connection
                    .post()
                    .get_string(field)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false))
            {
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("danger".into()));
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        format!("You need to enter all fields. The {field} field is missing.")
                            .into(),
                    ),
                );
            }
        }

        if !web_connection.session().contains("alertMessage") {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("warning".into()));

            // The Java `emailExists` check is commented out in
            // the source.

            let email = web_connection.post().get_string("email").unwrap_or_default();
            let credits = web_connection.post().get_string("credits").unwrap_or_default();
            let pixels = web_connection.post().get_string("pixels").unwrap_or_default();

            if !EmailUtil::is_valid_email_address(&email) {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("The email entered is not valid".into()),
                );
            } else if !is_numeric(&credits) {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        "The value supplied for credits is not a number".into(),
                    ),
                );
            } else if !is_numeric(&pixels) {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        "The value supplied for pixels is not a number".into(),
                    ),
                );
            }
        }
    }

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let mut player = match PlayerDao::get_details(id) {
        Some(player) => player,
        None => {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("danger".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("The user does not exist".into()),
            );
            return Ok(());
        }
    };

    let Some(session) = PlayerDao::get_details(web_connection.session().get_int("user.id"))
    else {
        // Java NPEs on a null session player.
        return Ok(());
    };

    let session_rank = session
        .get_rank()
        .map(|rank| rank.rank_id())
        .unwrap_or(0);
    let player_rank = player.get_rank().map(|rank| rank.rank_id()).unwrap_or(0);

    if session_rank <= player_rank {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "You cannot edit someone that has a equal or higher rank than you".into(),
            ),
        );
    } else {
        // Mirrors the Java `!client.session().contains("alertMessages")` check
        // (note the typo in the Java source).
        if web_connection.post().queries().len() > 0
            && !web_connection.session().contains("alertMessages")
        {
            player.set_figure(&web_connection.post().get_string("figure").unwrap_or_default());
            player.set_motto(&web_connection.post().get_string("motto").unwrap_or_default());
            player.set_credits(web_connection.post().get_string("credits").unwrap_or_default().parse().unwrap_or(0));
            player.set_email(&web_connection.post().get_string("email").unwrap_or_default());

            PlayerDao::save_details(
                player.get_id(),
                player.get_figure(),
                player.get_pool_figure(),
                player.get_sex(),
            );
            PlayerDao::save_motto(player.get_id(), player.get_motto());
            PlayerDao::save_currency(player.get_id(), player.get_credits());
            PlayerDao::save_email(player.get_id(), player.get_email());

            web_connection
                .session()
                .set("alertColour", SessionValue::Str("success".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("The user has been successfully saved".into()),
            );
        }
    }

    tpl.set("playerId", TemplateValue::of(player.get_id()));
    tpl.set("playerUsername", TemplateValue::of(player.get_name()));
    tpl.set("playerEmail", TemplateValue::of(player.get_email()));
    tpl.set("playerMotto", TemplateValue::of(player.get_motto()));
    tpl.set("playerPixels", TemplateValue::of(0));
    tpl.set("playerCredits", TemplateValue::of(player.get_credits()));
    tpl.set("playerFigure", TemplateValue::of(player.get_figure()));

    tpl.set("pageName", TemplateValue::of("Edit User"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
