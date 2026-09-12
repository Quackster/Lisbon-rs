//! Mirrors `org.alexdev.http.util.SessionUtil`.

use rand::Rng;

use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::web_connection::{SessionValue, WebConnection};

/// Mirrors `org.alexdev.http.util.SessionUtil`.
pub struct SessionUtil;

impl SessionUtil {
    pub const PLAYER: &'static str = "player";
    pub const USER_ID: &'static str = "user.id";
    pub const LOGGED_IN: &'static str = "authenticated";
    pub const LOGGED_IN_HOUSKEEPING: &'static str = "authenticatedHousekeeping";
    pub const MACHINE_ID: &'static str = "SECURITY_KEY";

    pub const REMEMBER_TOKEN_NAME: &'static str = "remember_token";
    pub const REMEMBER_TOKEN_AGE_SECONDS: i32 = 31 * 24 * 60 * 60;
    pub const REAUTHENTICATE_TIME: i32 = 30 * 60;

    /// Mirrors `login(WebConnection, String, String, boolean)`.
    pub fn login(
        web_connection: &WebConnection,
        username: &str,
        password: &str,
        delete_auth_variables: bool,
    ) -> bool {
        let mut details = PlayerDetails::new();
        let has_error;

        if username.trim().is_empty() || password.trim().is_empty() {
            has_error = true;
        } else {
            has_error = !Self::player_login(&mut details, username, password);
        }

        if has_error {
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("Incorrect username or password\n".to_string()),
            );

            if delete_auth_variables {
                web_connection.session().delete(Self::USER_ID);
                web_connection.session().delete(Self::LOGGED_IN);
            }
            return false;
        }

        web_connection
            .session()
            .set(Self::LOGGED_IN, SessionValue::Bool(true));
        web_connection
            .session()
            .set("captcha.invalid", SessionValue::Bool(false));
        web_connection.session().set(
            Self::USER_ID,
            SessionValue::Str(details.get_id().to_string()),
        );
        web_connection
            .session()
            .set("clientAuthenticate", SessionValue::Bool(false));
        web_connection.session().set(
            "lastRequest",
            SessionValue::Str(
                (DateUtil::get_current_time_seconds() as i64 + Self::REAUTHENTICATE_TIME as i64)
                    .to_string(),
            ),
        );

        let remember_me = web_connection
            .post()
            .get_string("_login_remember_me")
            .map(|value| value == "true")
            .unwrap_or(false);

        if remember_me {
            let mut bytes = [0u8; 16];
            rand::thread_rng().fill(&mut bytes);
            let remember_me_token = hex::encode(bytes);
            web_connection.cookies().set(
                Self::REMEMBER_TOKEN_NAME,
                &remember_me_token,
                DateUtil::get_current_time_seconds() as i64
                    + Self::REMEMBER_TOKEN_AGE_SECONDS as i64,
            );
            crate::dao::session_dao::SessionDao::set_remember_token(
                details.get_id(),
                &remember_me_token,
            );
        } else {
            web_connection
                .cookies()
                .set(Self::REMEMBER_TOKEN_NAME, "", 0);
        }

        web_connection.cookies().set("vote_stamp", "", 0);

        if details.is_banned().is_some() {
            web_connection.redirect("/account/banned");
        }

        true
    }

    /// Mirrors `logout(WebConnection)`.
    pub fn logout(web_connection: &WebConnection) {
        if web_connection.cookies().exists(Self::REMEMBER_TOKEN_NAME) {
            web_connection
                .cookies()
                .set(Self::REMEMBER_TOKEN_NAME, "", 0);
            if let Some(user_id) = web_connection
                .session()
                .get_string(Self::USER_ID)
                .and_then(|value| value.parse::<i32>().ok())
            {
                crate::dao::session_dao::SessionDao::clear_remember_token(user_id);
            }
        }

        web_connection.session().delete(Self::USER_ID);
        web_connection.session().delete(Self::LOGGED_IN);
        web_connection.session().delete("minimailLabel");
        web_connection.session().delete("lastBrowsedPage");
    }

    /// Mirrors `checkCookie(WebConnection)`.
    pub fn check_cookie(web_connection: &WebConnection) {
        if !web_connection.cookies().exists(Self::REMEMBER_TOKEN_NAME) {
            return;
        }

        let token = match web_connection.cookies().get(Self::REMEMBER_TOKEN_NAME) {
            Some(token) if !token.trim().is_empty() => token,
            _ => return,
        };

        let user_id = Self::get_remember_token_user_id(&token);

        if user_id > 0 {
            web_connection
                .session()
                .set(Self::LOGGED_IN, SessionValue::Bool(true));
            web_connection
                .session()
                .set("captcha.invalid", SessionValue::Bool(false));
            web_connection
                .session()
                .set(Self::USER_ID, SessionValue::Int(user_id));

            let uri = web_connection.request().uri();
            if uri == "/home" || uri == "/index" || uri == "/" {
                web_connection.redirect("/me");
            }
        } else {
            web_connection.session().delete(Self::USER_ID);
            web_connection.session().delete(Self::LOGGED_IN);
            web_connection
                .cookies()
                .set(Self::REMEMBER_TOKEN_NAME, "", 0);
        }
    }

    fn player_login(details: &mut PlayerDetails, username: &str, password: &str) -> bool {
        lisbon_server::dao::mysql::player_dao::PlayerDao::login(
            details,
            username,
            password,
        )
    }

    fn get_remember_token_user_id(token: &str) -> i32 {
        crate::dao::session_dao::SessionDao::get_remember_token(token)
    }
}
