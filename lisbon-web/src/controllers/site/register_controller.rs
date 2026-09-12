//! Mirrors `org.alexdev.http.controllers.site.RegisterController`.

use rand::Rng;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::dao::mysql::referred_dao::ReferredDao;
use lisbon_server::game::misc::figure::figure_manager::FigureManager;
use lisbon_server::game::player::player_manager::PlayerManager;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::figure_util::FigureUtil;

use crate::controllers::site::random_uuid;
use crate::dao::register_dao::RegisterDao;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::ResponseBuilder;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::captcha::Captcha;
use crate::util::email_util::EmailUtil;
use crate::util::html_util::HtmlUtil;
use crate::util::register_util::RegisterUtil;
use crate::util::session_util::SessionUtil;

/// Mirrors `register(WebConnection)`.
pub fn register(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/me");
        return Ok(());
    }

    let max_connections_per_ip =
        GameConfiguration::get_instance().get_integer("max.connections.per.ip");
    let ip_address = web_connection.get_ip_address();

    if PlayerDao::count_ip_address(&ip_address) >= max_connections_per_ip {
        web_connection
            .session()
            .set("alertMessage", SessionValue::Str("You already have enough accounts registered".to_string()));
        web_connection.redirect("/");
        return Ok(());
    }

    if web_connection.cookies().exists(SessionUtil::MACHINE_ID) {
        if let Some(machine_id) = web_connection.cookies().get(SessionUtil::MACHINE_ID) {
            if PlayerDao::count_machine_id(&format!("#{machine_id}"))
                >= max_connections_per_ip
            {
                web_connection
                    .session()
                    .set("alertMessage", SessionValue::Str("You already have enough accounts registered".to_string()));
                web_connection.redirect("/");
                return Ok(());
            }
        }
    }

    if GameConfiguration::get_instance().get_bool("registration.disabled") {
        let mut template = web_connection.template("register_disabled");
        template.render();
        return Ok(());
    }

    let mut referral = 0;

    if web_connection.get().contains("referral") {
        referral = web_connection.get().get_int("referral").unwrap_or(0);
    }

    if referral > 0 {
        web_connection.session().set("referral", SessionValue::Int(referral));
    }

    if web_connection.post().queries().len() > 3 {
        let check_empty = [
            "bean.avatarName",
            "bean.captchaResponse",
            "retypedPassword",
            "bean.email",
        ];

        for field in check_empty {
            if web_connection.post().contains(field)
                && web_connection
                    .post()
                    .get_string(field)
                    .map_or(false, |value| value.trim().is_empty())
            {
                web_connection
                    .session()
                    .set("captcha.invalid", SessionValue::Bool(false));
                web_connection.redirect("/register?errorCode=blank_fields");
                return Ok(());
            }
        }

        let mut username = String::new();
        let mut email = String::new();

        if web_connection.post().contains("registerUsername") {
            username = HtmlUtil::remove_html_tags(
                web_connection
                    .session()
                    .get_string("registerUsername")
                    .as_deref()
                    .unwrap_or_default(),
            );
        }

        if web_connection.post().contains("registerEmail") {
            email = HtmlUtil::remove_html_tags(
                web_connection
                    .session()
                    .get_string("registerEmail")
                    .as_deref()
                    .unwrap_or_default(),
            );
        }

        if web_connection.post().contains("bean.avatarName") {
            username = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.avatarName")
                    .as_deref()
                    .unwrap_or_default(),
            );
        }

        if web_connection.post().contains("bean.email") {
            email = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.email")
                    .as_deref()
                    .unwrap_or_default(),
            );
        }

        if web_connection.post().queries().len() > 10 {
            let password = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("retypedPassword")
                    .as_deref()
                    .unwrap_or_default(),
            );
            let day = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.day")
                    .as_deref()
                    .unwrap_or_default(),
            );
            let month = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.month")
                    .as_deref()
                    .unwrap_or_default(),
            );
            let year = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.year")
                    .as_deref()
                    .unwrap_or_default(),
            );
            let mut figure = String::new();
            let mut gender = String::new();

            if web_connection.post().contains("randomFigure") {
                let temp = HtmlUtil::remove_html_tags(
                    web_connection
                        .post()
                        .get_string("randomFigure")
                        .as_deref()
                        .unwrap_or_default(),
                );
                let temp = temp.chars().collect::<Vec<_>>();
                if temp.len() > 2 {
                    figure = temp[1..].iter().collect();
                    gender = temp[0].to_string();
                }
            } else {
                figure = HtmlUtil::remove_html_tags(
                    web_connection
                        .post()
                        .get_string("bean.figure")
                        .as_deref()
                        .unwrap_or_default(),
                );
                gender = HtmlUtil::remove_html_tags(
                    web_connection
                        .post()
                        .get_string("bean.gender")
                        .as_deref()
                        .unwrap_or_default(),
                );
            }

            web_connection
                .session()
                .set("registerUsername", SessionValue::Str(username.clone()));
            web_connection
                .session()
                .set("registerPassword", SessionValue::Str(password.clone()));
            web_connection.session().set(
                "registerShowPassword",
                SessionValue::Str(mask_password(&password)),
            );
            web_connection
                .session()
                .set("registerFigure", SessionValue::Str(figure.clone()));
            web_connection
                .session()
                .set("registerGender", SessionValue::Str(gender.clone()));
            web_connection
                .session()
                .set("registerEmail", SessionValue::Str(email.clone()));
            web_connection.session().set("registerDay", SessionValue::Str(day));
            web_connection.session().set("registerMonth", SessionValue::Str(month));
            web_connection.session().set("registerYear", SessionValue::Str(year));

            if !FigureManager::get_instance().validate_figure(&figure, &gender, false) {
                web_connection.redirect("/register?error=bad_look");
                return Ok(());
            }

            if !RegisterUtil::is_valid_name(&username) {
                web_connection.redirect("/register?error=bad_username");
                return Ok(());
            }

            if !RegisterUtil::is_valid_email(&email) {
                web_connection.session().set("email.invalid", SessionValue::Bool(true));
                web_connection.redirect("/register?error=bad_email");
                return Ok(());
            }
        }

        let captcha_response = HtmlUtil::remove_html_tags(
            web_connection
                .post()
                .get_string("bean.captchaResponse")
                .as_deref()
                .unwrap_or_default(),
        );

        let stored_captcha = web_connection
            .session()
            .get_string("captcha-text")
            .unwrap_or_default();

        if captcha_response != stored_captcha {
            web_connection.session().set("captcha.invalid", SessionValue::Bool(true));
            web_connection.redirect("/register?error=bad_captcha");
            return Ok(());
        }

        if web_connection.post().contains("bean.email") {
            email = HtmlUtil::remove_html_tags(
                web_connection
                    .post()
                    .get_string("bean.email")
                    .as_deref()
                    .unwrap_or_default(),
            );
            web_connection.session().set("registerEmail", SessionValue::Str(email.clone()));
        }

        if !RegisterUtil::is_valid_email(
            web_connection
                .session()
                .get_string("registerEmail")
                .as_deref()
                .unwrap_or_default(),
        ) {
            web_connection.session().set("email.invalid", SessionValue::Bool(true));
            web_connection.redirect("/register?error=bad_email");
            return Ok(());
        }

        let birthday = RegisterUtil::format_birthday(
            web_connection
                .session()
                .get_string("registerDay")
                .as_deref()
                .unwrap_or_default(),
            web_connection
                .session()
                .get_string("registerMonth")
                .as_deref()
                .unwrap_or_default(),
            web_connection
                .session()
                .get_string("registerYear")
                .as_deref()
                .unwrap_or_default(),
        );

        let birthday = match birthday {
            Some(birthday) => birthday,
            None => {
                web_connection.redirect("/register?error=bad_birthday");
                return Ok(());
            }
        };

        let hashed_password = PlayerManager::get_instance()
            .create_password(
                web_connection
                    .session()
                    .get_string("registerPassword")
                    .as_deref()
                    .unwrap_or_default(),
            );

        let user_id = RegisterDao::new_user(
            web_connection
                .session()
                .get_string("registerUsername")
                .as_deref()
                .unwrap_or_default(),
            &hashed_password,
            web_connection
                .session()
                .get_string("registerFigure")
                .as_deref()
                .unwrap_or_default(),
            web_connection
                .session()
                .get_string("registerGender")
                .as_deref()
                .unwrap_or_default(),
            web_connection
                .session()
                .get_string("registerEmail")
                .as_deref()
                .unwrap_or_default(),
            &birthday,
        );

        let activation_code = random_uuid();
        PlayerStatisticsDao::new_statistics(user_id, &activation_code);

        if GameConfiguration::get_instance().get_bool("email.smtp.enable") {
            let rendered = EmailUtil::render_registered(
                user_id,
                web_connection
                    .session()
                    .get_string("registerUsername")
                    .as_deref()
                    .unwrap_or_default(),
                web_connection
                    .session()
                    .get_string("registerEmail")
                    .as_deref()
                    .unwrap_or_default(),
                &activation_code,
            )
            .unwrap_or_default();
            EmailUtil::send(
                web_connection,
                web_connection
                    .session()
                    .get_string("registerEmail")
                    .as_deref()
                    .unwrap_or_default(),
                "Activate your account at Classic Habbo",
                &rendered,
            );
        }

        let latest_ip_address = PlayerDao::get_latest_ip(user_id);

        if latest_ip_address.is_empty() || latest_ip_address != ip_address {
            PlayerDao::log_ip_address(user_id, &ip_address);
        }

        referral = web_connection.session().get_int("referral");

        if referral > 0 {
            ReferredDao::add_referred(referral, user_id);
        }

        web_connection.session().delete("referral");
        web_connection.session().delete("captcha.invalid");

        web_connection.session().set("user.id", SessionValue::Int(user_id));
        web_connection.session().set("authenticated", SessionValue::Bool(true));

        web_connection.redirect("/welcome");
    } else {
        let mut template = web_connection.template("register");

        if web_connection.session().get_boolean("captcha.invalid")
            || web_connection.session().get_boolean("email.invalid")
        {
            if web_connection.session().get_boolean("captcha.invalid") {
                template.set("registerCaptchaInvalid", TemplateValue::of(true));
            }

            if web_connection.session().get_boolean("email.invalid") {
                template.set("registerEmailInvalid", TemplateValue::of(true));
            }

            template.set(
                "registerUsername",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerUsername")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerShowPassword",
                TemplateValue::of(mask_password(
                    web_connection
                        .session()
                        .get_string("registerPassword")
                        .as_deref()
                        .unwrap_or_default(),
                )),
            );
            template.set(
                "registerFigure",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerFigure")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerGender",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerGender")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerEmail",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerEmail")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerDay",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerDay")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerMonth",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerMonth")
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "registerYear",
                TemplateValue::of(
                    web_connection
                        .session()
                        .get_string("registerYear")
                        .unwrap_or_default(),
                ),
            );
        }

        template.set(
            "randomNum",
            TemplateValue::of(rand::thread_rng().gen_range(0..10000)),
        );
        template.set(
            "randomFemaleFigure1",
            TemplateValue::of(FigureUtil::get_random_figure(Some("F"), false)),
        );
        template.set(
            "randomFemaleFigure2",
            TemplateValue::of(FigureUtil::get_random_figure(Some("F"), false)),
        );
        template.set(
            "randomFemaleFigure3",
            TemplateValue::of(FigureUtil::get_random_figure(Some("F"), false)),
        );

        template.set(
            "randomMaleFigure1",
            TemplateValue::of(FigureUtil::get_random_figure(Some("M"), false)),
        );
        template.set(
            "randomMaleFigure2",
            TemplateValue::of(FigureUtil::get_random_figure(Some("M"), false)),
        );
        template.set(
            "randomMaleFigure3",
            TemplateValue::of(FigureUtil::get_random_figure(Some("M"), false)),
        );

        template.set(
            "referral",
            TemplateValue::of(web_connection.session().get_int("referral")),
        );

        template.render();
    }

    Ok(())
}

/// Mirrors `registerCancelled(WebConnection)`.
pub fn register_cancelled(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    web_connection.session().delete("referral");
    web_connection.session().delete("captcha.invalid");
    web_connection.redirect("/");

    Ok(())
}

/// Mirrors `captcha(WebConnection)`.
pub fn captcha(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let captcha_text = Captcha::generate_text(7);
    web_connection
        .session()
        .set("captcha-text", SessionValue::Str(captcha_text.clone()));

    let image = Captcha::generate_image(&captcha_text);
    let response =
        ResponseBuilder::create_with_content_type_bytes(http::StatusCode::OK, "image/png", image);

    web_connection.send(response);

    Ok(())
}

fn mask_password(password: &str) -> String {
    password.chars().map(|_| '*').collect()
}
