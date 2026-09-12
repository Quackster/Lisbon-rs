//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingRoomBadgesController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::badge_dao::BadgeDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::game::room::room_manager::RoomManager;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::util::housekeeping_util::HousekeepingUtil;
use crate::util::rcon_util::RconUtil;
use crate::routes::HOUSEKEEPING_PATH;
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
        "room_badges",
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    Ok(true)
}

fn send_room_badge_update() {
    RoomManager::get_instance().reload_badges();
    RconUtil::send_command(RconHeader::RefreshRoomBadges, HashMap::new());
}

/// Mirrors `badges(WebConnection)`.
pub fn badges(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection)? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/room_badges");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().queries().len() > 0 {
        let mut badges: HashMap<i32, Vec<String>> = HashMap::new();

        for (key, _value) in web_connection.post().get_values() {
            if !key.starts_with("roombadge-id-") {
                continue;
            }

            let values = key.strip_prefix("roombadge-id-").unwrap_or("");

            let room_id = web_connection
                .post()
                .get_int(&format!("roomad-{values}-roomid"))
                .unwrap_or(0);
            let badge_code = web_connection
                .post()
                .get_string(&format!("roomad-{values}-badge"))
                .unwrap_or_default();

            badges
                .entry(room_id)
                .or_default()
                .push(badge_code);
        }

        BadgeDao::update_badges(&badges);
        send_room_badge_update();

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "All badge rooms have been saved successfully!".into(),
            ),
        );
    }

    // The Java `catch` alert is not expressible because the
    // ported DAO methods do not fail.
    let room_entry_badges = RoomManager::get_instance().get_room_entry_badges();

    tpl.set(
        "roomBadges",
        TemplateValue::json(
serde_json::Value::Array(            room_entry_badges
                .iter()
                .map(|(room_id, badge_list)| {
                    serde_json::json!({
                        "roomId": room_id,
                        "badges": badge_list,
                    })
                })
                .collect::<Vec<_>>(),

        )        ),
    );
    tpl.set("util", TemplateValue::of(HousekeepingUtil));
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

    let mut tpl = web_connection.template("housekeeping/room_badges");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().queries().len() > 0 {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "All badge rooms have been saved successfully!".into(),
            ),
        );
    }

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("There was no badge selected to delete".into()),
        );
    } else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Successfully deleted the badge".into()),
        );

        let id = web_connection.get().get_string("id").unwrap_or_default();
        let data: Vec<&str> = id.split('_').collect();

        if data.len() >= 2 {
            BadgeDao::delete_room_badge(data[0], data[1]);
        }
    }

    send_room_badge_update();

    let room_entry_badges = RoomManager::get_instance().get_room_entry_badges();

    tpl.set(
        "roomBadges",
        TemplateValue::json(
serde_json::Value::Array(            room_entry_badges
                .iter()
                .map(|(room_id, badge_list)| {
                    serde_json::json!({
                        "roomId": room_id,
                        "badges": badge_list,
                    })
                })
                .collect::<Vec<_>>(),

        )        ),
    );
    tpl.set("util", TemplateValue::of(HousekeepingUtil));
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

    let mut tpl = web_connection.template("housekeeping/room_badges_create");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().get_values().len() > 0 {
        let room_id = web_connection.post().get_int("roomid").unwrap_or(0);
        let badge_code = web_connection.post().get_string("badgecode").unwrap_or_default();

        BadgeDao::create_entry_badge(room_id, &badge_code);

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Successfully created the room entry badge".into()),
        );

        send_room_badge_update();
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/room_badges"));

        return Ok(());
    }

    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
