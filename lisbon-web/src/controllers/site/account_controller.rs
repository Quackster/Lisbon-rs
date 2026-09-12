//! Mirrors `org.alexdev.http.controllers.site.AccountController`.

use chrono::Datelike;

use lisbon_server::dao::mysql::alerts_dao::AlertsDao;
use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::group_member_dao::GroupMemberDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::dao::mysql::tag_dao::TagDao;
use lisbon_server::game::alerts::alert_type::AlertType;
use lisbon_server::game::club::club_subscription::ClubSubscription;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::{DateUtil, LONG_DATE};

use crate::controllers::site::random_uuid;
use crate::dao::group_discussion_dao::GroupDiscussionDao;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::game::account::beginner_gift_manager::BeginnerGiftManager;
use crate::game::friends::friends_feed::FriendsFeed;
use crate::game::news::news_article::NewsArticle;
use crate::server::watchdog::{
    EVENTS, NEWS, NEWS_STAFF, RECOMMENDED_GROUPS, STAFF_PICK_GROUPS,
};
use crate::util::html_util::HtmlUtil;
use crate::util::session_util::SessionUtil;
use crate::util::tag_util::TagUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `submit(WebConnection)`.
pub fn submit(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let username = HtmlUtil::remove_html_tags(
        web_connection
            .post()
            .get_string("username")
            .as_deref()
            .unwrap_or_default(),
    );
    let password = HtmlUtil::remove_html_tags(
        web_connection
            .post()
            .get_string("password")
            .as_deref()
            .unwrap_or_default(),
    );

    if SessionUtil::login(web_connection, &username, &password, true) {
        web_connection.redirect("/security_check");
    } else {
        let remember_me = web_connection
            .post()
            .get_string("_login_remember_me")
            .map_or(false, |value| value == "true");

        let mut template = web_connection.template("account/submit");
        template.set(
            "rememberMe",
            TemplateValue::of(if remember_me { "true" } else { "false" }),
        );
        template.set("username", TemplateValue::of(username));
        template.render();
    }

    Ok(())
}

/// Mirrors `securityCheck(WebConnection)`.
pub fn security_check(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("/security_check");
    let redirect_path = if web_connection.session().contains("lastBrowsedPage") {
        web_connection
            .session()
            .get_string("lastBrowsedPage")
            .unwrap_or_else(|| "/me".to_string())
    } else {
        "/me".to_string()
    };
    template.set("redirectPath", TemplateValue::of(redirect_path));
    template.render();

    Ok(())
}

/// Mirrors `me(WebConnection)`.
pub fn me(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("me");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            web_connection.session().delete("user.id");
            web_connection.session().delete("authenticated");
            web_connection.redirect("/");
            return Ok(());
        }
    };

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    web_connection.session().set("page", SessionValue::Str("me".to_string()));
    web_connection.session().delete("captcha.invalid");

    if player_details.has_club_subscription() {
        let hc_days =
            (player_details.get_club_expiration() - DateUtil::get_current_time_seconds() as i64)
                / 86400;
        template.set("hcDays", TemplateValue::of(hc_days));
    }

    let include_unpublished = player_details
        .get_rank()
        .map_or(false, |rank| rank.rank_id() > 1);

    let article_list: Vec<NewsArticle> = if include_unpublished {
        NEWS_STAFF.read().iter().cloned().collect()
    } else {
        NEWS.read().iter().cloned().collect()
    };

    let mut articles: Vec<Option<NewsArticle>> = vec![None; 5];
    let mut index = 0;
    for article in article_list {
        if index < 5 {
            articles[index] = Some(article);
            index += 1;
        }
    }

    if articles[0].is_none() {
        articles[0] = Some(NewsArticle::new(
            0,
            "Installation Complete",
            0,
            "",
            "Welcome to your brand new Web installation!",
            "",
            DateUtil::get_current_time_seconds() as i64,
            "Rel22_commu_topstory_300x187.gif",
            "",
            "",
            "0",
            true,
            0,
            false,
        ));
    }

    for index in 0..5 {
        if articles[index].is_none() {
            articles[index] = Some(NewsArticle::new(
                0,
                "No news",
                0,
                "",
                "",
                "",
                DateUtil::get_current_time_seconds() as i64,
                "attention_topstory.png",
                "",
                "",
                "0",
                true,
                0,
                false,
            ));
        }

        template.set(
            &format!("article{}", index + 1),
            TemplateValue::of(articles[index].as_ref().unwrap()),
        );
    }

    let alerts = AlertsDao::get_alerts(player_details.get_id());
    let mut statistics_values = PlayerStatisticsDao::get_statistics(player_details.get_id());

    if statistics_values.is_empty() {
        PlayerStatisticsDao::new_statistics(player_details.get_id(), &random_uuid());
        statistics_values = PlayerStatisticsDao::get_statistics(player_details.get_id());
    }

    let statistics = PlayerStatisticManager::new(player_details.get_id(), statistics_values);

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
        let mut seconds = statistics.get_int_value(PlayerStatistic::NewbieGiftTime) as i64
            - DateUtil::get_current_time_seconds() as i64;

        if BeginnerGiftManager::progress(&player_details, &statistics) {
            seconds = statistics.get_int_value(PlayerStatistic::NewbieGiftTime) as i64
                - DateUtil::get_current_time_seconds() as i64;
        }

        if seconds < 0 {
            seconds = 0;
        }

        template.set(
            "newbieNextGift",
            TemplateValue::of(statistics.get_int_value(PlayerStatistic::NewbieGift)),
        );
        template.set("newbieGiftSeconds", TemplateValue::of(seconds));
    }

    if player_details.has_club_subscription() {
        if alerts
            .iter()
            .any(|alert| alert.get_alert_type() == AlertType::HcExpired)
        {
            AlertsDao::delete_alerts(player_details.get_id(), AlertType::HcExpired);
        }
    } else if !alerts
        .iter()
        .any(|alert| alert.get_alert_type() == AlertType::HcExpired)
    {
        if player_details.get_first_club_subscription() > 0 {
            AlertsDao::create_alert(player_details.get_id(), AlertType::HcExpired, "");
        }
    }

    if player_details.get_selected_room_id() == -1
        && statistics.get_int_value(PlayerStatistic::NewbieRoomLayout) != -1
    {
        statistics.set_long_value(PlayerStatistic::NewbieRoomLayout, -1);
    }

    let join_date = player_details.format_join_date("MM/dd");
    let current_date = DateUtil::get_date(
        DateUtil::get_current_time_seconds() as i64,
        "MM/dd",
    );

    if join_date == current_date
        && !(player_details.format_join_date("MM/dd/yyyy")
            == DateUtil::get_date(
                DateUtil::get_current_time_seconds() as i64,
                "MM/dd/yyy"
            ))
    {
        let birthday = DateUtil::get_date_time_from_timestamp(player_details.get_join_date());
        let now =
            DateUtil::get_date_time_from_timestamp(DateUtil::get_current_time_seconds() as i64);

        match (birthday, now) {
            (Some(birthday), Some(now)) => {
                // Mirrors `Period.between(...).getYears()` (derived from the
                // date parts; `chrono` `TimeDelta` has no `num_years`).
                let mut age = now.year() - birthday.year();
                if (now.month(), now.day()) < (birthday.month(), birthday.day()) {
                    age -= 1;
                }

                template.set("hasBirthday", TemplateValue::of(true));
                template.set("birthdayAge", TemplateValue::of(age));

                let suffix = match age % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                };
                template.set("birthdayPrefix", TemplateValue::of(suffix));
            }
            _ => {
                template.set("hasBirthday", TemplateValue::of(false));
            }
        }
    } else {
        template.set("hasBirthday", TemplateValue::of(false));
    }

    template.set(
        "tags",
        TemplateValue::of(TagDao::get_user_tags(player_details.get_id())),
    );
    template.set(
        "lastOnline",
        TemplateValue::of(DateUtil::get_friendly_date(player_details.get_last_online())),
    );
    template.set(
        "tagRandomQuestion",
        TemplateValue::of(TagUtil::get_random_question()),
    );

    template.set("events", TemplateValue::of(EVENTS.read().clone()));
    template.set(
        "groups",
        TemplateValue::of(GroupDao::get_joined_groups(
            web_connection.session().get_int("user.id"),
        )),
    );
    template.set(
        "alerts",
        TemplateValue::of(
            alerts
                .iter()
                .filter(|alert| !alert.is_disabled())
                .cloned()
                .collect::<Vec<_>>(),
        ),
    );
    template.set("recommendedGroups", TemplateValue::of(RECOMMENDED_GROUPS.read().clone()));
    template.set("staffPickGroups", TemplateValue::of(STAFF_PICK_GROUPS.read().clone()));

    FriendsFeed::create_friends_online(web_connection, &mut template);
    super::minimail_controller::append_messages(
        web_connection,
        &mut template,
        true,
        false,
        false,
        false,
        false,
        false,
    );

    let (pending_count, pending_groups) =
        GroupMemberDao::get_pending_members(player_details.get_id());
    template.set("pendingMembers", TemplateValue::of(pending_count));
    template.set("pendingGroups", TemplateValue::of(pending_groups));

    let (new_posts_amount, new_posts) = GroupDiscussionDao::get_new_group_messages(
        player_details.get_id(),
        player_details.get_last_online(),
    );
    template.set("newPostsAmount", TemplateValue::of(new_posts_amount));
    template.set("newPosts", TemplateValue::of(new_posts));

    template.set(
        "unreadGuestbookMessages",
        TemplateValue::of(statistics.get_int_value(PlayerStatistic::GuestbookUnreadMessages)),
    );
    template.render();

    ClubSubscription::count_member_days_details(&player_details, &statistics);

    let ip_address = web_connection.get_ip_address();
    let latest_ip_address = PlayerDao::get_latest_ip(player_details.get_id());

    if latest_ip_address.is_empty() || latest_ip_address != ip_address {
        PlayerDao::log_ip_address(player_details.get_id(), &ip_address);
    }

    if !web_connection.cookies().exists(SessionUtil::MACHINE_ID)
        || web_connection
            .cookies()
            .get(SessionUtil::MACHINE_ID)
            .map_or(true, |value| value != player_details.get_machine_id())
    {
        if !player_details.get_machine_id().is_empty() {
            web_connection.cookies().set(
                SessionUtil::MACHINE_ID,
                player_details.get_machine_id().replace('#', "").as_str(),
                2 * 86400,
            );
        }
    }

    Ok(())
}

/// Mirrors `welcome(WebConnection)`.
pub fn welcome(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("welcome");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return Ok(()),
    };

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    if !player_details.can_select_room() {
        web_connection.redirect("/me");
        return Ok(());
    }

    web_connection.session().set("page", SessionValue::Str("welcome".to_string()));
    template.render();

    Ok(())
}

/// Mirrors `reauthenticate(WebConnection)`.
pub fn reauthenticate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    if !web_connection.post().queries().is_empty() {
        let user_id = web_connection.session().get_int("user.id");

        if let Some(player_details) = PlayerDao::get_details(user_id) {
            let username = player_details.get_name().to_string();
            let password = web_connection
                .post()
                .get_string("password")
                .unwrap_or_default();

            if SessionUtil::login(web_connection, &username, &password, false) {
                let client_request = web_connection.session().get_string("clientRequest");
                let target = if web_connection.session().contains("clientRequest") {
                    client_request.as_deref().unwrap_or("/me")
                } else {
                    "/me"
                };
                web_connection.redirect(target);
                return Ok(());
            }
        }
    }

    web_connection.session().set("page", SessionValue::Str("reauthenticate".to_string()));

    let mut template = web_connection.template("account/reauthenticate");
    template.render();

    web_connection.session().delete("alertMessage");

    Ok(())
}

/// Mirrors `login_popup(WebConnection)`.
pub fn login_popup(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.session().set("page", SessionValue::Str("login_popup".to_string()));

    let mut template = web_connection.template("account/login");
    template.render();

    web_connection.session().delete("alertMessage");

    Ok(())
}

/// Mirrors `banned(WebConnection)`.
pub fn banned(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    web_connection.session().delete("lastBrowsedPage");

    let mut template = web_connection.template("account/banned");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return Ok(()),
    };

    let (reason, expiration) = match player_details.is_banned() {
        Some(ban) => ban,
        None => {
            web_connection.redirect("/me");
            return Ok(());
        }
    };

    web_connection.session().set("page", SessionValue::Str("banned".to_string()));

    let banned_message = format!(
        "You have been banned from {}. The reason for the ban is \"{}\". The ban will expire at {}.",
        GameConfiguration::get_instance().get_string("site.name"),
        reason,
        DateUtil::get_date(expiration, LONG_DATE)
    );

    template.set("bannedMsg", TemplateValue::of(banned_message));
    template.render();

    if !web_connection.cookies().exists(SessionUtil::MACHINE_ID)
        || web_connection
            .cookies()
            .get(SessionUtil::MACHINE_ID)
            .map_or(true, |value| value != player_details.get_machine_id())
    {
        if !player_details.get_machine_id().is_empty() {
            web_connection.cookies().set(
                SessionUtil::MACHINE_ID,
                player_details.get_machine_id().replace('#', "").as_str(),
                2 * 86400,
            );
        }
    }

    SessionUtil::logout(web_connection);

    Ok(())
}

/// Mirrors `logout(WebConnection)`.
pub fn logout(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    SessionUtil::logout(web_connection);

    web_connection.session().set("page", SessionValue::Str("logout".to_string()));

    let mut template = web_connection.template("account/logout");
    template.render();

    Ok(())
}

