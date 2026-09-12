//! Mirrors `org.alexdev.http.controllers.site.RecoveryController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::player_manager::PlayerManager;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::controllers::site::random_uuid;
use crate::dao::email_dao::EmailDao;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::email_util::EmailUtil;

/// Mirrors `forgot(WebConnection)`.
pub fn forgot(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !GameConfiguration::get_instance().get_bool("email.smtp.enable") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("account/email/account_forgot");

    if !web_connection.post().get_values().is_empty() {
        if web_connection.post().contains("actionList") {
            let email = web_connection
                .post()
                .get_string("ownerEmailAddress")
                .unwrap_or_default();

            if !web_connection.post().contains("ownerEmailAddress")
                || !EmailUtil::is_valid_email_address(&email)
                || !EmailDao::get_details_by_email(&email)
            {
                template.set("invalidForgetName", TemplateValue::of(true));
                template.render();
                return Ok(());
            }

            let mut sent_template = web_connection.template("account/email/sent");
            sent_template.render();
            return Ok(());
        }

        if web_connection.post().contains("actionForgot") {
            let username = web_connection
                .post()
                .get_string("forgottenpw-username")
                .unwrap_or_default();
            let email = web_connection
                .post()
                .get_string("forgottenpw-email")
                .unwrap_or_default();

            if username.trim().is_empty() {
                template.set("invalidForgetPassword", TemplateValue::of(true));
                template.render();
                return Ok(());
            }

            if email.trim().is_empty() {
                template.set("invalidForgetPassword", TemplateValue::of(true));
                template.render();
                return Ok(());
            }

            let details = EmailDao::get_details(&username, &email);

            if !EmailUtil::is_valid_email_address(&email) || details.is_none() {
                template.set("invalidForgetPassword", TemplateValue::of(true));
                template.render();
                return Ok(());
            }

            let details = details.unwrap();
            let recovery_code = random_uuid();
            let user_id = PlayerDao::get_id(&username);

            PlayerStatisticsDao::update_statistic(
                user_id,
                PlayerStatistic::ForgotPasswordCode,
                &recovery_code,
            );
            PlayerStatisticsDao::update_statistic(
                user_id,
                PlayerStatistic::ForgotRecoveryRequestedTime,
                &DateUtil::get_current_time_seconds().to_string(),
            );

            if GameConfiguration::get_instance().get_bool("email.smtp.enable") {
                let rendered = EmailUtil::render_password_recovery(
                    details.get_id(),
                    details.get_name(),
                    &recovery_code,
                )
                .unwrap_or_default();
                EmailUtil::send(web_connection, &email, "Password recovery at Classic Habbo", &rendered);
            }

            let mut sent_template = web_connection.template("account/email/sent");
            sent_template.render();
            return Ok(());
        }
    }

    web_connection.session().set("page", SessionValue::Str("recover".to_string()));
    template.render();

    Ok(())
}

/// Mirrors `recovery(WebConnection)`.
pub fn recovery(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !GameConfiguration::get_instance().get_bool("email.smtp.enable") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut user_id = web_connection.get().get_int("id").unwrap_or(0);

    let mut recovery_code = web_connection
        .get()
        .get_string("code")
        .unwrap_or_default();

    let mut template = web_connection.template("account/email/account_recovery");

    if web_connection.post().contains("user_id") && web_connection.post().contains("recovery_code") {
        user_id = web_connection.post().get_int("user_id").unwrap_or(0);
        recovery_code = web_connection
            .post()
            .get_string("recovery_code")
            .unwrap_or_default();
    }

    if (user_id == 0 || recovery_code.is_empty())
        || !EmailDao::recovery_exists(user_id, &recovery_code)
    {
        web_connection.session().set("alertMessage", SessionValue::Str("The recovery code was invalid".to_string()));
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("red".to_string()));
    } else {
        if web_connection.post().contains("password") && web_connection.post().contains("confirmpassword")
        {
            let password = web_connection
                .post()
                .get_string("password")
                .unwrap_or_default();
            let new_password = web_connection
                .post()
                .get_string("confirmpassword")
                .unwrap_or_default();

            if new_password != password {
                web_connection.session().set("alertMessage", SessionValue::Str("The passwords don't match".to_string()));
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("red".to_string()));
            } else if new_password.chars().count() < 6 {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("Password is too short, 6 characters minimum".to_string()),
                );
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("red".to_string()));
            } else {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        "Your password has been changed successfully.".to_string(),
                    ),
                );
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("green".to_string()));

                PlayerDao::set_password(
                    user_id,
                    &PlayerManager::get_instance().create_password(&new_password),
                );
                EmailDao::remove_recovery_code(user_id);
            }
        }

        template.set("recoveryCode", TemplateValue::of(recovery_code.clone()));
        template.set("userId", TemplateValue::of(user_id));
    }

    template.render();

    web_connection.session().delete("alertMessage");
    web_connection.session().delete("alertColour");

    Ok(())
}

/// Mirrors `activate(WebConnection)`.
pub fn activate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !GameConfiguration::get_instance().get_bool("email.smtp.enable") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.get().get_int("id").unwrap_or(0);

    let activation_code = web_connection
        .get()
        .get_string("code")
        .unwrap_or_default();

    let mut template = web_connection.template("account/email/account_activated");
    template.set("verifySuccess", TemplateValue::of(true));

    if user_id == 0 || activation_code.is_empty() {
        template.set("verifySuccess", TemplateValue::of(false));
    } else if !EmailDao::exists(user_id, &activation_code) {
        template.set("verifySuccess", TemplateValue::of(false));
    } else {
        EmailDao::activate(user_id, &activation_code);
    }

    template.render();

    Ok(())
}
