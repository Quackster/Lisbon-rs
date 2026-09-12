//! Mirrors `org.alexdev.http.util.EmailUtil`.

use lazy_static::lazy_static;
use regex::Regex;

use lettre::transport::Transport;

use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::email_dao::EmailDao;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::web_connection::{SessionValue, WebConnection};
use crate::havana_web::HavanaWeb;
use crate::template::twig_template::TwigTemplate;

const EMAIL_COOLDOWN: i64 = 2 * 60;

/// Mirrors `org.alexdev.http.util.EmailUtil`.
pub struct EmailUtil;

impl EmailUtil {
    /// Mirrors `send(WebConnection, String, String, String)`.
    pub fn send(
        web_connection: &WebConnection,
        target_email: &str,
        subject: &str,
        rendered_html: &str,
    ) -> bool {
        if !GameConfiguration::get_instance().get_bool("email.smtp.enable") {
            return true;
        }

        if !Self::is_valid_email_address(target_email) {
            return false;
        }

        if web_connection.session().contains("lastEmailTime") {
            let last_email_time =
                web_connection.session().get_int("lastEmailTime") as i64 + EMAIL_COOLDOWN;

            if last_email_time > DateUtil::get_current_time_seconds() as i64 {
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str(
                        "Please wait a few minutes before sending an email again".to_string(),
                    ),
                );
                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("red".to_string()));
                return false;
            }
        }

        web_connection.session().set(
            "lastEmailTime",
            SessionValue::Str(DateUtil::get_current_time_seconds().to_string()),
        );

        let target_email = target_email.to_string();
        let subject = subject.to_string();
        let rendered_html = rendered_html.to_string();

        HavanaWeb::get_executor().execute(move || {
            let config = GameConfiguration::get_instance();
            let host = config.get_string("email.smtp.host");
            let port = config.get_integer("email.smtp.port");
            let username = config.get_string("email.smtp.login.username");
            let password = config.get_string("email.smtp.login.password");
            let from_email = config.get_string("email.smtp.from.email");
            let from_name = config.get_string("email.smtp.from.name");

            let result: Result<(), Box<dyn std::error::Error>> = (|| {
                let from = lettre::message::Mailbox::new(
                    if from_name.is_empty() {
                        None
                    } else {
                        Some(from_name)
                    },
                    from_email.parse()?,
                );

                let mailer = lettre::transport::smtp::SmtpTransport::relay(&host)?
                    .credentials(
                        lettre::transport::smtp::authentication::Credentials::from((
                            username, password,
                        )),
                    )
                    .port(port as u16)
                    .timeout(Some(std::time::Duration::from_secs(5)))
                    .build();

                let email = lettre::message::MessageBuilder::new()
                    .from(from)
                    .to(target_email.parse()?)
                    .subject(subject)
                    .header(lettre::message::header::ContentType::TEXT_HTML)
                    .body(rendered_html)?;

                mailer.send(&email)?;

                Ok(())
            })();

            if let Err(error) = result {
                tracing::error!("email send failed: {error}");
            }
        });

        true
    }

    /// Mirrors `renderRegistered(int, String, String, String)`.
    pub fn render_registered(
        player_id: i32,
        player_name: &str,
        player_email: &str,
        activation_code: &str,
    ) -> Option<String> {
        let mut tpl = TwigTemplate::new(None);
        tpl.start("account/email/email_registered");
        tpl.set("playerId", TemplateValue::of(player_id));
        tpl.set("playerName", TemplateValue::of(player_name.to_string()));
        tpl.set("playerEmail", TemplateValue::of(player_email.to_string()));
        tpl.set("activationCode", TemplateValue::of(activation_code.to_string()));
        tpl.render_html().ok()
    }

    /// Mirrors `renderActivate(int, String, String, String)`.
    pub fn render_activate(
        player_id: i32,
        player_name: &str,
        player_email: &str,
        activation_code: &str,
    ) -> Option<String> {
        let mut tpl = TwigTemplate::new(None);
        tpl.start("account/email/email_activate");
        tpl.set("playerId", TemplateValue::of(player_id));
        tpl.set("playerName", TemplateValue::of(player_name.to_string()));
        tpl.set("playerEmail", TemplateValue::of(player_email.to_string()));
        tpl.set("activationCode", TemplateValue::of(activation_code.to_string()));
        tpl.render_html().ok()
    }

    /// Mirrors `renderPasswordRecovery(int, String, String)`.
    pub fn render_password_recovery(
        player_id: i32,
        player_name: &str,
        recovery_code: &str,
    ) -> Option<String> {
        let mut tpl = TwigTemplate::new(None);
        tpl.start("account/email/email_recovery");
        tpl.set("playerId", TemplateValue::of(player_id));
        tpl.set("playerName", TemplateValue::of(player_name.to_string()));
        tpl.set("recoveryCode", TemplateValue::of(recovery_code.to_string()));
        tpl.render_html().ok()
    }

    /// Mirrors `isAlreadyTradePass(int, String)`.
    pub fn is_already_trade_pass(user_id: i32, email: &str) -> bool {
        EmailDao::has_user_trade_pass(user_id, email)
    }

    /// Mirrors `EmailValidator.getInstance().isValid(String)` (commons-validator).
    pub fn is_valid_email_address(email: &str) -> bool {
        lazy_static! {
            static ref EMAIL_RE: Regex =
                Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
        }
        EMAIL_RE.is_match(email)
    }
}
