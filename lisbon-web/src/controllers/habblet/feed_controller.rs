//! Mirrors `org.alexdev.http.controllers.habblet.FeedController`.

use lisbon_server::dao::mysql::alerts_dao::AlertsDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::account::beginner_gift_manager::BeginnerGiftManager;

/// Mirrors `removeFeedItem(WebConnection)`.
pub fn remove_feed_item(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.session().get_boolean("authenticated") {
        connection.redirect("/");
        return Ok(());
    }

    let feed_item_index = connection.post().get_int("feedItemIndex").unwrap_or(-1);

    if feed_item_index != -1 {
        let user_id = connection.session().get_int("user.id");
        let account_alerts = AlertsDao::get_alerts(user_id);

        let alert_index = feed_item_index as usize;
        if alert_index < account_alerts.len() {
            let alert = &account_alerts[alert_index];
            AlertsDao::delete_alert(user_id, alert.get_id());
        }

        connection.send_string("");
    }

    Ok(())
}

/// Mirrors `nextGift(WebConnection)`.
pub fn nextgift(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.session().get_boolean("authenticated") {
        connection.send_string("");
        return Ok(());
    }

    let mut template = connection.template("habblet/nextgift");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            connection.send_string("");
            return Ok(());
        }
    };

    let statistics = PlayerStatisticManager::new(
        player_details.get_id(),
        PlayerStatisticsDao::get_statistics(player_details.get_id()),
    );

    template.set(
        "newbieRoomLayout",
        TemplateValue::of(statistics.get_int_value(PlayerStatistic::NewbieRoomLayout)),
    );
    template.set(
        "newbieNextGift",
        TemplateValue::of(statistics.get_int_value(PlayerStatistic::NewbieGift)),
    );

    if statistics.get_int_value(PlayerStatistic::NewbieRoomLayout) > 0
        && statistics.get_int_value(PlayerStatistic::NewbieGift) > 0
    {
        let mut seconds =
            statistics.get_int_value(PlayerStatistic::NewbieGiftTime)
                - DateUtil::get_current_time_seconds();

        if BeginnerGiftManager::progress(&player_details, &statistics) {
            seconds = statistics.get_int_value(PlayerStatistic::NewbieGiftTime)
                - DateUtil::get_current_time_seconds();
            template.set(
                "newbieNextGift",
                TemplateValue::of(statistics.get_int_value(PlayerStatistic::NewbieGift)),
            );
        }

        if seconds < 0 {
            seconds = 0;
        }

        template.set("newbieGiftSeconds", TemplateValue::of(seconds));
    }

    template.render();
    Ok(())
}

/// Mirrors `giftQueueHide(WebConnection)`.
pub fn giftqueue_hide(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.session().get_boolean("authenticated") {
        connection.send_string("");
        return Ok(());
    }

    let user_id = connection.session().get_int("user.id");

    let next_gift =
        PlayerStatisticsDao::get_statistic_long(user_id, PlayerStatistic::NewbieGift) as i32;

    if next_gift == 3 {
        PlayerStatisticsDao::update_statistic(user_id, PlayerStatistic::NewbieGift, "4");
    }

    connection.send_string("");
    Ok(())
}
