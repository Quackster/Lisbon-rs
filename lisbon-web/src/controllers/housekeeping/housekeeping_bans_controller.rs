//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingBansController`.

use lisbon_server::dao::mysql::ban_dao::BanDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_rank::PlayerRank;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

/// Mirrors `bans(WebConnection)`.
pub fn bans(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let mut current_page = 0;

    if web_connection.get().contains("page") {
        current_page = web_connection
            .get()
            .get_string("page")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
    }

    let mut sort_by = "banned_at".to_string();

    if web_connection.get().contains("sort") {
        let sort = web_connection.get().get_string("sort").unwrap_or_default();

        if sort == "banned_at" || sort == "banned_until" {
            sort_by = sort;
        }
    }

    let mut tpl = web_connection.template("housekeeping/users_bans");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    };

    if !HousekeepingManager::get_instance()
        .has_permission(
            player_details.get_rank().unwrap_or(PlayerRank::Rankless),
            "bans",
        )
    {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    tpl.set("pageName", TemplateValue::of("Bans"));
    tpl.set(
        "bans",
        TemplateValue::of(BanDao::get_active_bans(current_page, &sort_by)),
    );
    tpl.set(
        "nextBans",
        TemplateValue::of(BanDao::get_active_bans(current_page + 1, &sort_by)),
    );
    tpl.set(
        "previousBans",
        TemplateValue::of(BanDao::get_active_bans(current_page - 1, &sort_by)),
    );
    tpl.set("page", TemplateValue::of(current_page));
    tpl.set("sortBy", TemplateValue::of(sort_by));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
