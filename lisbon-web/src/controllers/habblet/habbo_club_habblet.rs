//! Mirrors `org.alexdev.http.controllers.habblet.HabboClubHabblet`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::alerts_dao::AlertsDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::alerts::alert_type::AlertType;
use lisbon_server::game::club::club_subscription::ClubSubscription;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::util::rcon_util::RconUtil;

fn option_number(web_connection: &WebConnection) -> i32 {
    web_connection
        .post()
        .get_string("optionNumber")
        .and_then(|value| value.parse().ok())
        .unwrap_or(1)
}

/// Mirrors `confirm(WebConnection)`.
pub fn confirm(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let option_number = option_number(web_connection);

    if option_number < 0 || option_number > 4 {
        return Ok(());
    }

    let (club_credits, club_days) = ClubSubscription::choice_data(option_number);

    let mut template = web_connection.template("habblet/habboClubConfirm");
    template.set("clubCredits", TemplateValue::of(club_credits));
    template.set("clubDays", TemplateValue::of(club_days));
    template.set("clubType", TemplateValue::of(option_number));
    template.render();
    Ok(())
}

/// Mirrors `subscribe(WebConnection)`.
pub fn subscribe(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        return Ok(());
    }

    let option_number = option_number(web_connection);
    let (credits, _days) = ClubSubscription::choice_data(option_number);

    let mut template = web_connection.template("habblet/habboClubSubscribe");

    let mut player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return Ok(()),
    };

    if player_details.get_credits() < credits {
        template.set(
            "subscribeMsg",
            TemplateValue::of(
                "You don't have enough credits to complete the subscription purchase.",
            ),
        );
    } else {
        template.set(
            "subscribeMsg",
            TemplateValue::of(format!(
                "Congratulations! You have successfully subscribed to {} Club.",
                GameConfiguration::get_instance().get_string("site.name")
            )),
        );

        let first_time = player_details.get_first_club_subscription() == 0;

        ClubSubscription::subscribe_club(&mut player_details, option_number);
        PlayerStatisticsDao::update_statistic(
            player_details.get_id(),
            PlayerStatistic::ClubMemberTimeUpdated,
            &(DateUtil::get_current_time_seconds() as i64
                + ClubSubscription::get_club_gift_seconds())
                .to_string(),
        );

        if player_details.is_online() {
            RconUtil::send_command(
                RconHeader::RefreshClub,
                HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
            );

            if first_time {
                RconUtil::send_command(
                    RconHeader::RefreshHand,
                    HashMap::from([
                        ("userId".to_string(), player_details.get_id().to_string()),
                    ]),
                );
            }
        }
    }

    template.render();
    Ok(())
}

/// Mirrors `endDate(WebConnection)`.
pub fn enddate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("habblet/habboClubEnddate");

    if web_connection.session().get_boolean("authenticated") {
        let user_id = web_connection.session().get_int("user.id");
        let player_details = match PlayerDao::get_details(user_id) {
            Some(details) => details,
            None => return Ok(()),
        };

        if player_details.has_club_subscription() {
            let hc_days = (player_details.get_club_expiration()
                - DateUtil::get_current_time_seconds() as i64)
                / 86_400;
            template.set("hcDays", TemplateValue::of(hc_days));
        }
    }

    template.render();
    Ok(())
}

/// Mirrors `reminderRemove(WebConnection)`.
pub fn reminder_remove(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => return Ok(()),
    };
    AlertsDao::disable_alerts(player_details.get_id(), AlertType::HcExpired);

    web_connection.send_string("");
    Ok(())
}
