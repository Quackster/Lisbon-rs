//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingInfobusController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::infobus_dao::InfobusDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::infobus::infobus_poll_data::InfobusPollData;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::util::html_util::HtmlUtil;
use crate::util::piechart::pie_chart::PieChart;
use crate::util::piechart::slice::{Color, Slice};
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::rcon_util::RconUtil;
use crate::util::session_util::SessionUtil;

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

/// Mirrors `polls(WebConnection)`.
pub fn polls(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/infobus_polls");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    tpl.set("pageName", TemplateValue::of("View Infobus Polls"));
    tpl.set("infobusPolls", TemplateValue::of(InfobusDao::get_infobus_polls()));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `create_polls(WebConnection)`.
pub fn create_polls(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/infobus_polls_create");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().get_values().len() > 0 {
        let question = web_connection.post().get_string("question").unwrap_or_default();

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Infobus poll has been created successfully".into()),
        );

        let user_id = web_connection.session().get_int("user.id");
        let Some(player_details) = PlayerDao::get_details(user_id) else {
            // Java NPEs on a null player.
            return Ok(());
        };

        let mut infobus_poll_data = InfobusPollData::new(&question);
        infobus_poll_data.add_answers(web_connection.post().get_array("answers[]"));
        InfobusDao::create_infobus_poll(player_details.get_id(), &infobus_poll_data);

        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    }

    // The Java `catch` block swallows errors; the ported DAO
    // methods do not fail.

    tpl.set("pageName", TemplateValue::of("Create Infobus Poll"));
    tpl.set(
        "oneHourLater",
        TemplateValue::of(DateUtil::get_date(
            DateUtil::get_current_time_seconds() as i64 + 3600,
            "yyyy-MM-dd'T'HH:mm",
        )),
    );
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `delete(WebConnection)`.
pub fn delete(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/articles");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let Some(poll) = InfobusDao::get(id) else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The infobus poll does not exist".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    };

    if poll.get_initiated_by() != player_details.get_id() {
        if !HousekeepingManager::get_instance().has_permission(
            player_details.get_rank().unwrap_or(PlayerRank::Rankless),
            "infobus/delete_any",
        ) {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("danger".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("No permission to delete other polls".into()),
            );
            web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
            return Ok(());
        }
    }

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        "infobus/delete_own",
    ) {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("No permission to delete".into()),
        );
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("There was no infobus poll selected to delete".into()),
        );
    } else {
        let answers = InfobusDao::get_answers(poll.get_id());
        let total_answers: i32 = answers.values().sum();

        if total_answers > 0 {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("danger".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("You can't delete a poll with answers".into()),
            );
            web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
            return Ok(());
        }

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Successfully deleted the infobus poll".into()),
        );

        InfobusDao::delete(id);
        InfobusDao::clear_answers(id);
    }

    web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
    Ok(())
}

/// Mirrors `send_poll(WebConnection)`.
pub fn send_poll(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let Some(poll) = InfobusDao::get(id) else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The infobus poll does not exist".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    };

    web_connection
        .session()
        .set("alertColour", SessionValue::Str("warning".into()));
    web_connection.session().set(
        "alertMessage",
        SessionValue::Str("The infobus poll request has been sent".into()),
    );

    RconUtil::send_command(
        RconHeader::InfobusPoll,
        HashMap::from([("pollId".to_string(), poll.get_id().to_string())]),
    );

    web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
    Ok(())
}

/// Mirrors `edit(WebConnection)`.
pub fn edit(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/infobus_polls_edit");
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
            SessionValue::Str("There was no infobus poll selected to edit".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    }

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let Some(infobus_poll) = InfobusDao::get(id) else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The infobus poll does not exist".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    };

    if web_connection.post().queries().len() > 0 {
        let question = web_connection.post().get_string("question").unwrap_or_default();

        let answers = InfobusDao::get_answers(infobus_poll.get_id());
        let total_answers: i32 = answers.values().sum();

        if total_answers > 0 {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("danger".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("You can't edit the poll if it has answers".into()),
            );
        } else {
            // The Java active-poll check is commented out in the
            // source.

            web_connection
                .session()
                .set("alertColour", SessionValue::Str("success".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("The infobus poll was successfully saved".into()),
            );

            let mut infobus_poll_data = InfobusPollData::new(&question);
            infobus_poll_data.add_answers(web_connection.post().get_array("answers[]"));
            InfobusDao::save_infobus_poll(infobus_poll.get_id(), &infobus_poll_data);
        }

        // The Java `REFRESH_INFOBUS_POLLS` command is commented
        // out in the source.
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    }

    // The Java `pollDate` line is commented out in the source.
    tpl.set("poll", TemplateValue::of(infobus_poll));

    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `view_results(WebConnection)`.
pub fn view_results(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/infobus_polls_view");
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
            SessionValue::Str("There was no infobus poll selected to edit".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    }

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let Some(infobus_poll) = InfobusDao::get(id) else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The infobus poll does not exist".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    };

    tpl.set("poll", TemplateValue::of(infobus_poll.clone()));

    let answers = InfobusDao::get_answers(infobus_poll.get_id());
    let total_answers: i32 = answers.values().sum();

    let mut slices = Vec::new();
    if total_answers > 0 {
        let poll_answers = infobus_poll.get_poll_data().get_answers();
        let mut i = 0;

        for (answer_key, vote_count) in &answers {
            let color = match i % 5 {
                0 => Color::new(0, 0, 255, 255),
                1 => Color::new(255, 0, 0, 255),
                2 => Color::new(255, 255, 0, 255),
                3 => Color::new(255, 192, 203, 255),
                _ => Color::new(255, 200, 87, 255),
            };

            let label = poll_answers
                .get(*answer_key as usize)
                .cloned()
                .unwrap_or_default();
            let value = if *vote_count > 0 {
                total_answers / *vote_count
            } else {
                0
            };

            slices.push(Slice::new(label, value as f64, color));
            i += 1;
        }
    }

    let chart = PieChart::new(Vec::new(), slices);
    let image_data = HtmlUtil::encode_to_string(chart.get_image(), "PNG")
        .map(|encoded| format!("data:image/png;base64,{encoded}"))
        .unwrap_or_default();

    tpl.set("imageData", TemplateValue::of(image_data));
    tpl.set("noAnswers", TemplateValue::of(total_answers == 0));

    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `clear_results(WebConnection)`.
pub fn clear_results(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("There was no infobus poll selected to edit".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    }

    let id = web_connection.get().get_int("id").unwrap_or(0);
    let Some(infobus_poll) = InfobusDao::get(id) else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The infobus poll does not exist".into()),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
        return Ok(());
    };

    web_connection
        .session()
        .set("alertColour", SessionValue::Str("success".into()));
    web_connection.session().set(
        "alertMessage",
        SessionValue::Str("The infobus poll has had all answers cleared".into()),
    );

    InfobusDao::clear_answers(infobus_poll.get_id());
    web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
    Ok(())
}

/// Mirrors `close_event(WebConnection)`.
pub fn close_event(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    web_connection
        .session()
        .set("alertColour", SessionValue::Str("success".into()));
    web_connection.session().set(
        "alertMessage",
        SessionValue::Str("The infobus status has been sent".into()),
    );

    RconUtil::send_command(RconHeader::InfobusEndEvent, HashMap::new());
    web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
    Ok(())
}

/// Mirrors `door_status(WebConnection)`.
pub fn door_status(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "infobus")? {
        return Ok(());
    }

    RconUtil::send_command(
        RconHeader::InfobusDoorStatus,
        HashMap::from([(
            "doorStatus".to_string(),
            web_connection.get().get_int("status").unwrap_or(0).to_string(),
        )]),
    );

    web_connection
        .session()
        .set("alertColour", SessionValue::Str("success".into()));
    web_connection.session().set(
        "alertMessage",
        SessionValue::Str("The infobus door status has been sent".into()),
    );

    web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/infobus_polls"));
    Ok(())
}
