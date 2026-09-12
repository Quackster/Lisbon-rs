//! Mirrors `org.alexdev.http.controllers.site.ClubController`.

use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::club::club_subscription::ClubSubscription;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::catalogue::catalogue_manager::CatalogueManager;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::xss_util::XssUtil as XSSUtil;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}


/// Mirrors `club(WebConnection)`.
pub fn club(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    web_connection.session().set("page", SessionValue::Str("credits".to_string()));
    renderclub(web_connection);

    Ok(())
}

/// Mirrors `renderclub(WebConnection)`.
pub fn renderclub(web_connection: &WebConnection) {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("club");

    for index in 0..3 {
        let (credits, days) = ClubSubscription::choice_data(index + 1);

        template.set(
            &format!("clubChoiceCredits{}", index + 1),
            TemplateValue::of(credits),
        );
        template.set(
            &format!("clubChoiceDays{}", index + 1),
            TemplateValue::of(days),
        );
    }

    if web_connection.session().get_boolean("authenticated") {
        if let Some(player_details) = template
            .get("playerDetails")
            .and_then(|value| PlayerDetails::from_json(value.value()))
        {
            let statistic_manager = PlayerStatisticManager::new(
                player_details.get_id(),
                PlayerStatisticsDao::get_statistics(player_details.get_id()),
            );

            if player_details.has_club_subscription() {
                let hc_days =
                    (player_details.get_club_expiration() - DateUtil::get_current_time_seconds() as i64)
                        / 86400;
                template.set("hcDays", TemplateValue::of(hc_days));

                let days =
                    statistic_manager.get_long_value(PlayerStatistic::ClubMemberTime) / 86400;
                let since_months = if days > 0 { days / 31 } else { 0 };

                template.set("hcSinceMonths", TemplateValue::of(since_months));
            }
        }
    }

    let last_club_gift_month = if web_connection.session().contains("lastClubGiftMonth") {
        web_connection.session().get_int("lastClubGiftMonth")
    } else {
        1
    };

    appendgiftdata(&mut template, last_club_gift_month, 0, web_connection);
    template.render();
}

/// Mirrors `habboClubGift(WebConnection)`.
pub fn habbo_club_gift(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.post().contains("month") || !connection.post().contains("catalogpage") {
        connection.send_string("");
        return Ok(());
    }

    if !connection
        .post()
        .get_string("month")
        .map_or(false, |value| is_numeric(&value))
        || !connection
            .post()
            .get_string("catalogpage")
            .map_or(false, |value| is_numeric(&value))
    {
        connection.send_string("");
        return Ok(());
    }

    let month = connection.post().get_int("month").unwrap_or(0);
    let catalogpage = connection.post().get_int("catalogpage").unwrap_or(0);

    let mut template = connection.template("habblet/habboclubgift");
    appendgiftdata(&mut template, month, catalogpage, connection);
    template.render();

    Ok(())
}

/// Mirrors `appendgiftdata(Template, int, int, WebConnection)`.
fn appendgiftdata(
    template: &mut impl Template,
    month: i32,
    _catalogpage: i32,
    connection: &WebConnection,
) {
    XSSUtil::clear(connection);

    let mut gift_order = ClubSubscription::gift_order();
    gift_order.insert(0, "club_sofa".to_string());

    let mut position = month - 1;

    if position >= gift_order.len() as i32 {
        position = 0;
    }

    let mut next_sprite_gift = gift_order[0].clone();

    if let Some(gift) = gift_order.get(position as usize) {
        next_sprite_gift = gift.clone();
    }

    let mut pages: Vec<i32> = Vec::new();

    let mut catalogpage = 0;

    if (5..=8).contains(&month) {
        catalogpage = 1;
    }

    if (9..=12).contains(&month) {
        catalogpage = 2;
    }

    if (13..=16).contains(&month) {
        catalogpage = 3;
    }

    if (17..=20).contains(&month) {
        catalogpage = 4;
    }

    if (21..=23).contains(&month) {
        catalogpage = 5;
    }

    if catalogpage == 0 {
        pages.extend([1, 2, 3, 4, 5]);
    }

    if catalogpage == 1 {
        pages.extend([5, 6, 7, 8, 9]);
    }

    if catalogpage == 2 {
        pages.extend([9, 10, 11, 12, 13]);
    }

    if catalogpage == 3 {
        pages.extend([13, 14, 15, 16, 17]);
    }

    if catalogpage == 4 {
        pages.extend([17, 18, 19, 20, 21]);
    }

    if catalogpage == 5 {
        pages.extend([19, 20, 21, 22, 23]);
    }

    let definition = CatalogueManager::get_instance().get_catalogue_item(&next_sprite_gift);

    template.set("pages", TemplateValue::of(pages));
    template.set("currentPage", TemplateValue::of(month));
    template.set("lastPage", TemplateValue::of(gift_order.len()));
    template.set("item", TemplateValue::of(definition));

    connection.session().set("lastClubGiftMonth", SessionValue::Int(month));
}

/// Mirrors `clubTryout(WebConnection)`.
pub fn club_tryout(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("club_tryout");

    for index in 0..3 {
        let (credits, days) = ClubSubscription::choice_data(index + 1);

        template.set(
            &format!("clubChoiceCredits{}", index + 1),
            TemplateValue::of(credits),
        );
        template.set(
            &format!("clubChoiceDays{}", index + 1),
            TemplateValue::of(days),
        );
    }

    if web_connection.session().get_boolean("authenticated") {
        if let Some(player_details) = template
            .get("playerDetails")
            .and_then(|value| PlayerDetails::from_json(value.value()))
        {
            let statistic_manager = PlayerStatisticManager::new(
                player_details.get_id(),
                PlayerStatisticsDao::get_statistics(player_details.get_id()),
            );

            if player_details.has_club_subscription() {
                let hc_days = (player_details.get_club_expiration()
                    - DateUtil::get_current_time_seconds() as i64)
                    / 86400;
                template.set("hcDays", TemplateValue::of(hc_days));

                let days =
                    statistic_manager.get_long_value(PlayerStatistic::ClubMemberTime) / 86400;
                let since_months = if days > 0 { days / 31 } else { 0 };

                template.set("hcSinceMonths", TemplateValue::of(since_months));
            }

            template.set("figure", TemplateValue::of(player_details.get_figure()));
            template.set("sex", TemplateValue::of(player_details.get_sex()));
        }
    }

    web_connection.session().set("page", SessionValue::Str("credits".to_string()));
    template.render();

    Ok(())
}
