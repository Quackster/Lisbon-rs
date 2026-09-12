//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingAdsController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::advertisements_dao::AdvertisementsDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::ads::ad_manager::AdManager;
use lisbon_server::game::ads::advertisement::Advertisement;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::rcon_util::RconUtil;
use crate::util::session_util::SessionUtil;

fn check_permission(
    web_connection: &WebConnection,
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
        "room_ads",
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    Ok(true)
}

/// Mirrors `roomads(WebConnection)`.
pub fn roomads(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection)? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/room_ads");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().queries().len() > 0 {
        let mut advertisement_list: Vec<Advertisement> = Vec::new();

        for (key, value) in web_connection.post().get_values() {
            if !key.starts_with("roomad-id-") {
                continue;
            }

            let room_id = web_connection
                .post()
                .get_int(&format!("roomad-{value}-roomid"))
                .unwrap_or(0);
            let is_loading_ad = web_connection
                .post()
                .contains(&format!("roomad-{value}-loading-ad"))
                && web_connection
                    .post()
                    .get_string(&format!("roomad-{value}-loading-ad"))
                    .map(|string| string.eq_ignore_ascii_case("on"))
                    .unwrap_or(false);
            let is_enabled = web_connection
                .post()
                .contains(&format!("roomad-{value}-enabled"))
                && web_connection
                    .post()
                    .get_string(&format!("roomad-{value}-enabled"))
                    .map(|string| string.eq_ignore_ascii_case("on"))
                    .unwrap_or(false);
            let image = web_connection
                .post()
                .get_string(&format!("roomad-{value}-image"))
                .unwrap_or_default();
            let url = web_connection
                .post()
                .get_string(&format!("roomad-{value}-url"))
                .unwrap_or_default();

            let Some(advertisement_id) = value.parse::<i32>().ok() else {
                continue;
            };

            advertisement_list.push(Advertisement::new(
                advertisement_id,
                is_loading_ad,
                room_id,
                image,
                url,
                is_enabled,
            ));
        }

        AdvertisementsDao::update_ads(&advertisement_list);
        AdManager::reset();

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "All room ads have been saved successfully!".into(),
            ),
        );

        RconUtil::send_command(RconHeader::RefreshAds, HashMap::new());
    }

    let mut advertisements = AdManager::get_instance().get_ads();
    advertisements.sort_by_key(|advertisement| advertisement.get_id());

    tpl.set("pageName", TemplateValue::of("Room Ads"));
    tpl.set("roomAds", TemplateValue::of(advertisements));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `delete(WebConnection)`.
pub fn delete(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection)? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/room_ads");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let id = web_connection.get().get_int("id").unwrap_or(0);
    AdvertisementsDao::delete_ad(id);

    web_connection
        .session()
        .set("alertColour", SessionValue::Str("danger".into()));
    web_connection.session().set(
        "alertMessage",
        SessionValue::Str("Room ad has been deleted successfully".into()),
    );

    RconUtil::send_command(RconHeader::RefreshAds, HashMap::new());

    AdManager::reset();
    let advertisements = AdManager::get_instance().get_ads();

    tpl.set("pageName", TemplateValue::of("Room Ads"));
    tpl.set("roomAds", TemplateValue::of(advertisements));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `create(WebConnection)`.
pub fn create(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection)? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/room_ads_create");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().get_values().len() > 0 {
        let room_id = web_connection.post().get_int("roomid").unwrap_or(0);
        let url = web_connection.post().get_string("url").unwrap_or_default();
        let image = web_connection.post().get_string("image").unwrap_or_default();
        let is_enabled = web_connection
            .post()
            .contains("enabled")
            && web_connection
                .post()
                .get_string("enabled")
                .map(|string| string.eq_ignore_ascii_case("on"))
                .unwrap_or(false);
        let is_room_loading_ad = web_connection
            .post()
            .contains("loading-ad")
            && web_connection
                .post()
                .get_string("loading-ad")
                .map(|string| string.eq_ignore_ascii_case("on"))
                .unwrap_or(false);

        AdvertisementsDao::create(room_id, &url, &image, is_enabled, is_room_loading_ad);
        AdManager::reset();

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Room ad has been created successfully".into()),
        );

        RconUtil::send_command(RconHeader::RefreshAds, HashMap::new());
    }

    tpl.set("pageName", TemplateValue::of("Room Ads"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
