//! Mirrors `org.alexdev.http.util.RegisterUtil`.

use chrono::NaiveDate;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;

use crate::util::email_util::EmailUtil;

/// Mirrors `org.alexdev.http.util.RegisterUtil`.
pub struct RegisterUtil;

impl RegisterUtil {
    /// Mirrors `isValidName(String)`.
    pub fn is_valid_name(username: &str) -> bool {
        Self::get_name_error_code(username) == 0
    }

    /// Mirrors `getNameErrorCode(String)`.
    pub fn get_name_error_code(username: &str) -> i32 {
        if WordfilterManager::filter_sentence(username) != username {
            6
        } else if !Self::has_allowed_characters(
            &username.to_lowercase(),
            "1234567890qwertyuiopasdfghjklzxcvbnm-+=?!@:.,$",
        ) {
            5
        } else if username.eq_ignore_ascii_case("admin")
            || username.eq_ignore_ascii_case("mod")
            || username.eq_ignore_ascii_case("staff")
            || username.eq_ignore_ascii_case("moderator")
            || username.eq_ignore_ascii_case("vip")
            || username.to_lowercase().starts_with("admin-")
            || username.to_lowercase().starts_with("admin=")
            || username.to_lowercase().starts_with("mod-")
            || username.to_lowercase().starts_with("mod=")
            || username.to_lowercase().starts_with("bot-")
            || username.to_lowercase().starts_with("bot=")
            || username.to_lowercase().starts_with("vip=")
            || username.to_lowercase().starts_with("vip-")
        {
            4
        } else if username.chars().count() > 24 {
            3
        } else if username.is_empty() {
            2
        } else if Self::get_id(username) > 0 {
            1
        } else {
            0
        }
    }

    /// Mirrors `NameCheckController.hasAllowedCharacters(String, String)`.
    fn has_allowed_characters(str: &str, allowed: &str) -> bool {
        for c in str.chars() {
            if !allowed.contains(c) {
                return false;
            }
        }
        true
    }

    fn get_id(username: &str) -> i32 {
        PlayerDao::get_id(username)
    }

    fn get_by_email(email: &str) -> i32 {
        PlayerDao::get_by_email(email)
    }

    /// Mirrors `isValidEmail(String)`.
    pub fn is_valid_email(email: &str) -> bool {
        if !EmailUtil::is_valid_email_address(email) {
            return false;
        }

        if Self::get_by_email(email) > 0 {
            return false;
        }

        true
    }

    /// Mirrors `formatBirthday(String, String, String)`.
    pub fn format_birthday(day: &str, month: &str, year: &str) -> Option<String> {
        let day_value = day.parse::<i32>().ok()?;
        let month_value = month.parse::<i32>().ok()?;
        let year_value = year.parse::<i32>().ok()?;

        let valid =
            NaiveDate::from_ymd_opt(year_value, month_value as u32, day_value as u32).is_some();
        if valid {
            Some(format!("{day_value:02}.{month_value:02}.{year_value:04}"))
        } else {
            None
        }
    }
}
