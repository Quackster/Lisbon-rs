//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::player_rank::PlayerRank;

use crate::dao::housekeeping::housekeeping_player_dao::HousekeepingPlayerDao;
use crate::dao::housekeeping_dao::HousekeepingDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::game::housekeeping::housekeeping_stats::HousekeepingStats;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

/// Mirrors `dashboard(WebConnection)`.
pub fn dashboard(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    // If they are logged in, send them to the /me page
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        let mut tpl = web_connection.template("housekeeping/login");
        tpl.render_html().ok();
    } else {
        let mut current_page = 0;

        if web_connection.get().contains("page") {
            current_page = web_connection
                .get()
                .get_string("page")
                .and_then(|value| value.parse().ok())
                .unwrap_or(0);
        }

        let zero_coins_flag = web_connection.get().contains("zerocoins");

        let mut sort_by = "created_at".to_string();

        if web_connection.get().contains("sort") {
            let sort = web_connection.get().get_string("sort").unwrap_or_default();

            if sort == "last_online" || sort == "created_at" {
                sort_by = sort;
            }
        }

        let mut tpl = web_connection.template("housekeeping/dashboard");
        tpl.set(
            "housekeepingManager",
            TemplateValue::of(HousekeepingManager::get_instance()),
        );

        tpl.set("pageName", TemplateValue::of("Dashboard"));
        tpl.set(
            "players",
            TemplateValue::of(HousekeepingPlayerDao::get_players(
                current_page,
                zero_coins_flag,
                &sort_by,
            )),
        );
        tpl.set(
            "nextPlayers",
            TemplateValue::of(HousekeepingPlayerDao::get_players(
                current_page + 1,
                zero_coins_flag,
                &sort_by,
            )),
        );
        tpl.set(
            "previousPlayers",
            TemplateValue::of(HousekeepingPlayerDao::get_players(
                current_page - 1,
                zero_coins_flag,
                &sort_by,
            )),
        );
        tpl.set("page", TemplateValue::of(current_page));
        tpl.set("sortBy", TemplateValue::of(sort_by));
        let stats = HousekeepingStats::new(
            HousekeepingDao::get_user_count(),
            HousekeepingDao::get_inventory_items_count(),
            HousekeepingDao::get_room_item_count(),
            HousekeepingDao::get_group_count(),
            HousekeepingDao::get_pet_count(),
            HousekeepingDao::get_photo_count(),
        );
        tpl.set("stats", TemplateValue::of(stats));
        tpl.set("zeroCoinsFlag", TemplateValue::of(zero_coins_flag));
        tpl.render_html().ok();

        // Delete alert after it's been rendered
        web_connection.session().delete("alertMessage");
    }
    Ok(())
}

/// Mirrors `login(WebConnection)`.
pub fn login(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let field_check = ["hkusername", "hkpassword"];

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
                SessionValue::Str("You need to enter both your email and password".into()),
            );
            web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
            return Ok(());
        }
    }

    let username = web_connection.post().get_string("hkusername").unwrap_or_default();
    let password = web_connection.post().get_string("hkpassword").unwrap_or_default();

    let mut player_details = PlayerDetails::new();

    if !PlayerDao::login(&mut player_details, &username, &password) {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection
            .session()
            .set("alertMessage", SessionValue::Str("You have entered invalid details".into()));
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        "root/login",
    ) {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("warning".into()));
        web_connection
            .session()
            .set("alertMessage", SessionValue::Str("You don't have permission".into()));
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    web_connection
        .session()
        .set(SessionUtil::LOGGED_IN_HOUSKEEPING, SessionValue::Bool(true));
    web_connection.session().set(
        SessionUtil::USER_ID,
        SessionValue::Str(player_details.get_id().to_string()),
    );
    web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
    Ok(())
}

/// Mirrors `logout(WebConnection)`.
pub fn logout(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection
            .session()
            .set("alertMessage", SessionValue::Str("Successfully logged out!".into()));
        web_connection
            .session()
            .set(SessionUtil::LOGGED_IN_HOUSKEEPING, SessionValue::Bool(false));
    }

    web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
    Ok(())
}
