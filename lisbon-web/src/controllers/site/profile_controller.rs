//! Mirrors `org.alexdev.http.controllers.site.ProfileController`.

use std::collections::HashMap;

use rand::Rng;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::dao::mysql::wardrobe_dao::WardrobeDao;
use lisbon_server::game::misc::figure::figure_manager::FigureManager;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::player_manager::PlayerManager;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::string_util::StringUtil;

use crate::dao::session_dao::SessionDao;
use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::ResponseBuilder;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::email_util::EmailUtil;
use crate::util::html_util::HtmlUtil;
use crate::util::rcon_util::RconUtil;
use crate::util::session_util::SessionUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

fn is_numeric(value: Option<&str>) -> bool {
    value.map_or(false, |value| !value.is_empty() && value.chars().all(|character| character.is_ascii_digit()))
}

/// Mirrors `profile(WebConnection)`.
pub fn profile(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let statistics_values = PlayerStatisticsDao::get_statistics(user_id);
    let _statistics = PlayerStatisticManager::new(user_id, statistics_values);

    web_connection.session().set("page", SessionValue::Str("me".to_string()));

    let mut tab = 0;

    if web_connection.get().contains("tab") {
        if is_numeric(web_connection.get().get_string("tab").as_deref()) {
            tab = web_connection
                .get()
                .get_string("tab")
                .and_then(|value| value.parse().ok())
                .unwrap_or(0);
        }
    }

    match tab {
        1 => {
            let mut template = web_connection.template("profile/change_looks");
            profile_flash(&mut template, web_connection);
        }
        2 => {
            let mut template = web_connection.template("profile/change_preferences");
            preferences(&mut template, web_connection);

            template.set("settingsSavedAlert", TemplateValue::of(false));

            if web_connection.session().contains("settings.saved.successfully") {
                template.set("settingsSavedAlert", TemplateValue::of(true));
            }

            template.set(
                "randomNumber",
                TemplateValue::of(rand::thread_rng().gen_range(0..i32::MAX)),
            );
            template.render();
        }
        3 | 4 => {
            let mut template = web_connection.template(
                if tab == 3 {
                    "profile/change_email"
                } else {
                    "profile/change_password"
                },
            );

            template.set("settingsSavedAlert", TemplateValue::of(false));

            if web_connection.session().contains("settings.saved.successfully") {
                template.set("settingsSavedAlert", TemplateValue::of(true));
            }

            template.set(
                "randomNumber",
                TemplateValue::of(rand::thread_rng().gen_range(0..i32::MAX)),
            );
            template.render();
        }
        5 => {
            let mut template = web_connection.template("profile/friend_management");
            super::friend_management_controller::friendmanagement(
                &mut template,
                web_connection,
                30,
                1,
                -1,
                None,
            );
        }
        _ => {}
    }

    web_connection.session().delete("settings.saved.successfully");
    web_connection.session().delete("alertMessage");
    web_connection.session().delete("alertColour");

    Ok(())
}

/// Mirrors `changelooks(Template, WebConnection)`.
fn changelooks(template: &mut impl Template, web_connection: &WebConnection) {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return;
    }

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return,
    };

    if !player_details.has_club_subscription() {
        web_connection.redirect("/");
        return;
    }

    let wardrobe_list = WardrobeDao::get_wardrobe(player_details.get_id());

    let wardrobe_slot_1 = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == 1);
    let wardrobe_slot_2 = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == 2);
    let wardrobe_slot_3 = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == 3);
    let wardrobe_slot_4 = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == 4);
    let wardrobe_slot_5 = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == 5);

    template.set("wardrobe1", TemplateValue::of(false));
    template.set("wardrobe2", TemplateValue::of(false));
    template.set("wardrobe3", TemplateValue::of(false));
    template.set("wardrobe4", TemplateValue::of(false));
    template.set("wardrobe5", TemplateValue::of(false));

    for (slot, index) in [
        (wardrobe_slot_1, 1),
        (wardrobe_slot_2, 2),
        (wardrobe_slot_3, 3),
        (wardrobe_slot_4, 4),
        (wardrobe_slot_5, 5),
    ] {
        if let Some(wardrobe) = slot {
            template.set(
                &format!("wardrobe{index}"),
                TemplateValue::of(true),
            );
            template.set(
                &format!("wardrobeUrl{index}"),
                TemplateValue::of(HtmlUtil::create_figure_link(
                    wardrobe.get_figure(),
                    wardrobe.get_sex()
                )),
            );
            template.set(
                &format!("wardrobeFigure{index}"),
                TemplateValue::of(wardrobe.get_figure().to_string()),
            );
            template.set(
                &format!("wardrobeSex{index}"),
                TemplateValue::of(wardrobe.get_sex().to_string()),
            );
        }
    }

    let figure_has_club = if !player_details.has_club_subscription() {
        FigureManager::get_instance().validate_figure_code(
            player_details.get_figure(),
            player_details.get_sex(),
            false,
        ) == 6
    } else {
        false
    };

    template.set("figureHasClub", TemplateValue::of(figure_has_club));
}

/// Mirrors `passwordupdate(WebConnection)`.
pub fn passwordupdate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/change_password");

    let mut player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            template.render();
            return Ok(());
        }
    };

    let mut logout = false;

    let current_password = web_connection
        .post()
        .get_string("currentpassword")
        .unwrap_or_default();
    let new_password = web_connection
        .post()
        .get_string("newpassword")
        .unwrap_or_default();
    let new_password_confirm = web_connection
        .post()
        .get_string("newpasswordconfirm")
        .unwrap_or_default();
    let captcha = web_connection
        .post()
        .get_string("captcha")
        .unwrap_or_default();

    if current_password.trim().is_empty()
        || new_password.trim().is_empty()
        || new_password_confirm.trim().is_empty()
        || captcha.trim().is_empty()
    {
        web_connection.session().set("alertMessage", SessionValue::Str("Please enter all fields".to_string()));
        web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
    } else {
        let username = player_details.get_name().to_string();
        if !PlayerDao::login(
            &mut player_details,
            &username,
            &current_password,
        ) {
            web_connection.session().set("alertMessage", SessionValue::Str("Your current password is invalid".to_string()));
            web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
        } else if new_password.chars().count() < 6 {
            web_connection.session().set("alertMessage", SessionValue::Str("Password is too short, 6 characters minimum".to_string()));
            web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
        } else if new_password != new_password_confirm {
            web_connection.session().set("alertMessage", SessionValue::Str("The passwords don't match".to_string()));
            web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
        } else {
            let captcha_text = web_connection
                .session()
                .get_string("captcha-text")
                .unwrap_or_default();

            if captcha != captcha_text {
                web_connection.session().set("alertMessage", SessionValue::Str("The security code was invalid, please try again.".to_string()));
                web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
            } else {
                web_connection.session().set("alertMessage", SessionValue::Str("Your password has been changed successfully. You will need to login again.".to_string()));
                web_connection.session().set("alertColour", SessionValue::Str("green".to_string()));

                PlayerDao::set_password(
                    player_details.get_id(),
                    &PlayerManager::get_instance().create_password(&new_password),
                );
                logout = true;
            }
        }
    }

    template.set(
        "randomNumber",
        TemplateValue::of(rand::thread_rng().gen_range(0..i32::MAX)),
    );
    template.render();

    web_connection.session().delete("alertMessage");
    web_connection.session().delete("alertColour");
    web_connection.session().delete("captcha-text");

    if logout {
        SessionUtil::logout(web_connection);
    }

    Ok(())
}

/// Mirrors `emailupdate(WebConnection)`.
pub fn emailupdate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let mut player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.redirect("/profile?tab=3");
            return Ok(());
        }
    };

    let password = web_connection
        .post()
        .get_string("password")
        .unwrap_or_default();
    let email = web_connection
        .post()
        .get_string("email")
        .unwrap_or_default();
    let captcha = web_connection
        .post()
        .get_string("captcha")
        .unwrap_or_default();

    if password.trim().is_empty() || captcha.trim().is_empty() {
        web_connection.session().set("alertMessage", SessionValue::Str("Please enter all fields".to_string()));
        web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
    } else {
        let username = player_details.get_name().to_string();
        if !PlayerDao::login(
            &mut player_details,
            &username,
            &password,
        ) {
            web_connection.session().set("alertMessage", SessionValue::Str("Your current password is invalid".to_string()));
            web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
        } else if !EmailUtil::is_valid_email_address(&email) {
            web_connection.session().set("alertMessage", SessionValue::Str("The email you entered is invalid".to_string()));
            web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
        } else {
            let captcha_text = web_connection
                .session()
                .get_string("captcha-text")
                .unwrap_or_default();

            if captcha != captcha_text {
                web_connection.session().set("alertMessage", SessionValue::Str("The security code was invalid, please try again.".to_string()));
                web_connection.session().set("alertColour", SessionValue::Str("red".to_string()));
            } else {
                web_connection.session().set("alertMessage", SessionValue::Str("Your email has been changed successfully.".to_string()));
                web_connection.session().set("alertColour", SessionValue::Str("green".to_string()));

                if player_details.get_email() != email {
                    let activation_code = crate::controllers::site::random_uuid();

                    if GameConfiguration::get_instance().get_bool("email.smtp.enable") {
                        let rendered = EmailUtil::render_activate(
                            player_details.get_id(),
                            player_details.get_name(),
                            &email,
                            &activation_code,
                        )
                        .unwrap_or_default();

                        if EmailUtil::send(web_connection, &email, "Activate your account at Classic Habbo", &rendered) {
                            PlayerStatisticsDao::update_statistic(
                                player_details.get_id(),
                                PlayerStatistic::ActivationCode,
                                &activation_code,
                            );

                            if GameConfiguration::get_instance().get_bool("trade.email.verification") {
                                if player_details.is_trade_enabled() {
                                    SessionDao::save_trade(player_details.get_id(), false);

                                    RconUtil::send_command(
                                        RconHeader::RefreshTradeSetting,
                                        HashMap::from([
                                            ("userId".to_string(), player_details.get_id().to_string()),
                                            ("tradeEnabled".to_string(), "0".to_string()),
                                        ]),
                                    );
                                }
                            }

                            PlayerDao::set_email(player_details.get_id(), &email);
                        }
                    }
                }
            }
        }
    }

    web_connection.redirect("/profile?tab=3");
    web_connection.session().delete("captcha-text");

    Ok(())
}

/// Mirrors `characterupdate(WebConnection)`.
pub fn characterupdate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let expected_fields = ["figureData", "newGender"];

    for field in expected_fields {
        if !web_connection.post().contains(field)
            || web_connection
                .post()
                .get_string(field)
                .map_or(true, |value| value.is_empty())
        {
            web_connection.redirect("/profile");
            return Ok(());
        }
    }

    let new_figure = web_connection
        .post()
        .get_string("figureData")
        .unwrap_or_default();
    let new_gender = web_connection
        .post()
        .get_string("newGender")
        .unwrap_or_default();

    let user_id = web_connection.session().get_int("user.id");
    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.redirect("/profile");
            return Ok(());
        }
    };

    let validate_figure_code = FigureManager::get_instance().validate_figure_code(
        &new_figure,
        &new_gender,
        player_details.has_club_subscription(),
    );

    if validate_figure_code > 0 {
        web_connection.redirect("/profile");
        return Ok(());
    }

    if !new_gender.starts_with('M') && !new_gender.starts_with('F') {
        web_connection.redirect("/profile");
        return Ok(());
    }

    PlayerDao::save_details(
        player_details.get_id(),
        &new_figure,
        player_details.get_pool_figure(),
        &new_gender,
    );

    if player_details.is_online() {
        RconUtil::send_command(
            RconHeader::RefreshLooks,
            HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
        );
    }

    web_connection.session().set("settings.saved.successfully", SessionValue::Str(String::new()));
    web_connection.redirect("/profile");

    Ok(())
}

/// Mirrors `action(WebConnection)`.
pub fn action(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.redirect("/profile");

    Ok(())
}

/// Mirrors `club(WebConnection)`.
pub fn club(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.session().set("page", SessionValue::Str("me".to_string()));
    super::club_controller::renderclub(web_connection);

    Ok(())
}

/// Mirrors `preferences(Template, WebConnection)`.
pub fn preferences(template: &mut impl Template, web_connection: &WebConnection) {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return;
    }

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return,
    };

    template.set("onlineStatusEnabled", TemplateValue::of(""));
    template.set("onlineStatusDisabled", TemplateValue::of(""));
    template.set("followFriendEnabled", TemplateValue::of(""));
    template.set("followFriendDisabled", TemplateValue::of(""));
    template.set("profileVisibleEnabled", TemplateValue::of(""));
    template.set("profileVisibleDisabled", TemplateValue::of(""));
    template.set("allowFriendRequests", TemplateValue::of(""));
    template.set("wordFilterSetting", TemplateValue::of(""));

    if player_details.is_online_status_visible() {
        template.set("onlineStatusEnabled", TemplateValue::of("checked=\"checked\""));
    } else {
        template.set("onlineStatusDisabled", TemplateValue::of("checked=\"checked\""));
    }

    if player_details.does_allow_stalking() {
        template.set("followFriendEnabled", TemplateValue::of("checked=\"checked\""));
    } else {
        template.set("followFriendDisabled", TemplateValue::of("checked=\"checked\""));
    }

    if player_details.is_profile_visible() {
        template.set("profileVisibleEnabled", TemplateValue::of("checked=\"checked\""));
    } else {
        template.set("profileVisibleDisabled", TemplateValue::of("checked=\"checked\""));
    }

    if player_details.is_allow_friend_requests() {
        template.set("allowFriendRequests", TemplateValue::of("checked=\"true\""));
    } else {
        template.set("allowFriendRequests", TemplateValue::of(""));
    }

    if player_details.is_word_filter_enabled() {
        template.set("wordFilterSetting", TemplateValue::of(""));
    } else {
        template.set("wordFilterSetting", TemplateValue::of("checked=\"true\""));
    }
}

/// Mirrors `profileupdate(WebConnection)`.
pub fn profileupdate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let mut motto = web_connection
        .post()
        .get_string("motto")
        .unwrap_or_default();
    let profile_visibility = web_connection
        .post()
        .get_string("visibility")
        .as_deref()
        == Some("EVERYONE");
    let online_status_visibility = web_connection
        .post()
        .get_string("showOnlineStatus")
        .as_deref()
        == Some("true");
    let word_filter_enabled = web_connection
        .post()
        .get_string("wordFilterSetting")
        .as_deref()
        != Some("false");
    let allow_friend_requests = web_connection
        .post()
        .get_string("allowFriendRequests")
        .as_deref()
        == Some("true");
    let allow_friend_stalking = web_connection
        .post()
        .get_string("followFriendSetting")
        .as_deref()
        == Some("true");

    if motto.chars().count() > 32 {
        motto = motto.chars().take(32).collect();
    }

    SessionDao::save_preferences(
        &motto,
        profile_visibility,
        online_status_visibility,
        word_filter_enabled,
        allow_friend_requests,
        allow_friend_stalking,
        user_id,
    );

    web_connection.session().set("settings.saved.successfully", SessionValue::Str("true".to_string()));
    web_connection.redirect("/profile?tab=2");

    Ok(())
}

/// Mirrors `wardrobeStore(WebConnection)`.
pub fn wardrobe_store(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    if !is_numeric(web_connection.post().get_string("slot").as_deref()) {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let slot_id = web_connection.post().get_int("slot").unwrap_or(0);

    let figure = StringUtil::filter_input(
        web_connection
            .post()
            .get_string("figure")
            .as_deref()
            .unwrap_or_default(),
        true,
    );
    let mut sex = StringUtil::filter_input(
        web_connection
            .post()
            .get_string("gender")
            .as_deref()
            .unwrap_or_default(),
        true,
    );

    if sex.trim().is_empty() {
        sex = "M".to_string();
    }

    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.redirect("/");
            return Ok(());
        }
    };

    if !FigureManager::get_instance().validate_figure(&figure, &sex, player_details.has_club_subscription()) {
        web_connection.redirect("/");
        return Ok(());
    }

    if slot_id < 1 || slot_id > 5 {
        web_connection.redirect("/");
        return Ok(());
    }

    let wardrobe_list = WardrobeDao::get_wardrobe(user_id);
    let wardrobe_data = wardrobe_list
        .iter()
        .find(|wardrobe| wardrobe.get_slot_id() == slot_id);

    if wardrobe_data.is_none() {
        WardrobeDao::add_wardrobe(user_id, slot_id, &figure, &sex.to_uppercase());
    } else {
        WardrobeDao::update_wardrobe(user_id, slot_id, &figure, &sex.to_uppercase());
    }

    let response = ResponseBuilder::create_with_content_type(
        "application/json",
        format!(
            "{{\"slot\":\"{slot_id}\",\"u\":\"{}\",\"f\":\"{figure}\",\"g\":77}}",
            HtmlUtil::create_figure_link(&figure, &sex)
        ),
    );
    web_connection.send(response);

    Ok(())
}

/// Mirrors `profile_flash(Template, WebConnection)`.
pub fn profile_flash(template: &mut impl Template, web_connection: &WebConnection) {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return;
    }

    let user_id = web_connection.session().get_int("user.id");

    let statistics_values = PlayerStatisticsDao::get_statistics(user_id);
    let _statistics = PlayerStatisticManager::new(user_id, statistics_values);

    web_connection.session().set("page", SessionValue::Str("me".to_string()));

    let _tab = if web_connection.get().contains("tab")
        && is_numeric(web_connection.get().get_string("tab").as_deref())
    {
        web_connection
            .get()
            .get_string("tab")
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    } else {
        0
    };

    changelooks(template, web_connection);

    if web_connection.session().contains("settings.saved.successfully") {
        template.set("settingsSavedAlert", TemplateValue::of("true"));
    }

    template.set(
        "randomNumber",
        TemplateValue::of(rand::thread_rng().gen_range(0..i32::MAX)),
    );
    template.render();

    web_connection.session().delete("settings.saved.successfully");
    web_connection.session().delete("alertMessage");
    web_connection.session().delete("alertColour");
}
