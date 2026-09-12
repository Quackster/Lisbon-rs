//! Mirrors `net.h4bbo.lisbon.util.DateUtil`.
//!
//! Java `SimpleDateFormat` patterns are converted to `chrono` format strings on
//! the fly by [`java_to_chrono`].

use chrono::{DateTime, Local};

pub const LONG_DATE: &str = "dd-MM-yyyy HH:mm:ss";
pub const CAMERA_DATE: &str = "dd/MM/yy HH:mm";
pub const SHORT_DATE: &str = "dd-MM-yyyy";
pub const SHORT_DATE_TIME: &str = "dd-MM-yyyy hh:mm a";

/// Converts a Java `SimpleDateFormat` pattern into a `chrono` format string.
fn java_to_chrono(pattern: &str) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let mut i = 0usize;

    while i < chars.len() {
        let c = chars[i];
        if matches!(c, 'd' | 'M' | 'y' | 'H' | 'h' | 'm' | 's' | 'a' | 'S' | 'E') {
            let mut j = i;
            while j < chars.len() && chars[j] == c {
                j += 1;
            }
            let run = j - i;
            match (c, run) {
                ('M', 3) => out.push_str("%b"),
                ('M', _) => out.push_str("%m"),
                ('y', 2) => out.push_str("%y"),
                ('y', _) => out.push_str("%Y"),
                ('S', _) => out.push_str("%3f"),
                ('E', _) => out.push_str("%a"),
                ('d', _) => out.push_str("%d"),
                ('H', _) => out.push_str("%H"),
                ('h', _) => out.push_str("%I"),
                ('m', _) => out.push_str("%M"),
                ('s', _) => out.push_str("%S"),
                ('a', _) => out.push_str("%p"),
                _ => {}
            }
            i = j;
        } else {
            // Literal characters (space, '-', ':', ',', etc.) pass through.
            out.push(c);
            i += 1;
        }
    }

    out
}

/// Build a `Local` [`DateTime`] from a unix timestamp in seconds.
fn from_epoch_seconds(time: i64) -> DateTime<Local> {
    let dt = DateTime::from_timestamp(time, 0)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());
    dt.with_timezone(&Local)
}

/// Mirrors `net.h4bbo.lisbon.util.DateUtil`.
pub struct DateUtil;

impl DateUtil {
    /// Mirrors `getCurrentDate(format)` — now, formatted.
    pub fn get_current_date(format: &str) -> String {
        Local::now().format(&java_to_chrono(format)).to_string()
    }

    /// Mirrors `getDate(time, format)` — a unix-seconds instant, formatted.
    pub fn get_date(time: i64, format: &str) -> String {
        from_epoch_seconds(time)
            .format(&java_to_chrono(format))
            .to_string()
    }

    /// Mirrors `getFriendlyDate(time)`.
    pub fn get_friendly_date(time: i64) -> String {
        let dt = from_epoch_seconds(time);
        let s = dt
            .format(&java_to_chrono("MMM dd, yyyy hh:mm:ss a"))
            .to_string();
        s.replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getFromFormat(format, date)`.
    pub fn get_from_format(format: &str, date: &str) -> i64 {
        match DateTime::parse_from_str(date, &java_to_chrono(format)) {
            Ok(dt) => dt.timestamp(),
            Err(_) => 0,
        }
    }

    /// Mirrors `getReadableTimestamp(timestamp)`.
    pub fn get_readable_timestamp(timestamp: i64) -> Option<String> {
        let different = Local::now().timestamp_millis() - (timestamp * 1000);

        let seconds_in_milli = 1000i64;
        let minutes_in_milli = seconds_in_milli * 60;
        let hours_in_milli = minutes_in_milli * 60;
        let days_in_milli = hours_in_milli * 24;

        let mut different = different;
        let elapsed_days = different / days_in_milli;
        different = different % days_in_milli;

        let elapsed_hours = different / hours_in_milli;
        different = different % hours_in_milli;

        let elapsed_minutes = different / minutes_in_milli;
        different = different % minutes_in_milli;

        let elapsed_seconds = different / seconds_in_milli;
        Some(format!(
            "{} days, {} hours, {} minutes, {} seconds",
            elapsed_days, elapsed_hours, elapsed_minutes, elapsed_seconds
        ))
    }

    /// Mirrors `getReadableSeconds(input)`.
    pub fn get_readable_seconds(input: i64) -> Option<String> {
        let uptime = input * 1000;
        let days = uptime / (1000 * 60 * 60 * 24);
        let hours = (uptime - days * (1000 * 60 * 60 * 24)) / (1000 * 60 * 60);
        let minutes =
            (uptime - days * (1000 * 60 * 60 * 24) - hours * (1000 * 60 * 60)) / (1000 * 60);
        let seconds =
            (uptime - days * (1000 * 60 * 60 * 24) - hours * (1000 * 60 * 60) - minutes * (1000 * 60))
                / 1000;

        Some(format!(
            "{} day(s), {} hour(s), {} minute(s) and {} second(s)",
            days, hours, minutes, seconds
        ))
    }

    /// Mirrors `getMarketplaceReadableSeconds(input)`.
    pub fn get_marketplace_readable_seconds(input: i64) -> Option<String> {
        let uptime = input * 1000;
        let days = uptime / (1000 * 60 * 60 * 24);
        let hours = (uptime - days * (1000 * 60 * 60 * 24)) / (1000 * 60 * 60);
        let minutes =
            (uptime - days * (1000 * 60 * 60 * 24) - hours * (1000 * 60 * 60)) / (1000 * 60);
        let _ = (uptime - days * (1000 * 60 * 60 * 24) - hours * (1000 * 60 * 60) - minutes * (1000 * 60))
            / 1000;

        let mut r = String::new();
        if days == 1 {
            r.push_str(&format!("{} day<br>", days));
        } else {
            r.push_str(&format!("{} days<br>", days));
        }

        if hours == 1 {
            r.push_str(&format!("{} hour", hours));
        } else {
            r.push_str(&format!("{} hours", hours));
        }

        if days < 1 {
            return Some(format!("<font color=\"red\">{}</font>", r));
        }

        Some(r)
    }

    /// Mirrors `getDateTimeFromTimestamp(timestamp)` — returns `None` for 0.
    pub fn get_date_time_from_timestamp(timestamp: i64) -> Option<DateTime<Local>> {
        if timestamp == 0 {
            return None;
        }
        Some(from_epoch_seconds(timestamp))
    }

    /// Mirrors `getShortDate()`.
    pub fn get_short_date() -> String {
        Self::get_current_date(SHORT_DATE)
    }

    /// Mirrors `getShortDate(time)` (the Java method ignores `time`).
    pub fn get_short_date_with_time(_time: i64) -> Option<String> {
        Some(Self::get_current_date(SHORT_DATE))
    }

    /// Mirrors `getDateAsString(time)`.
    pub fn get_date_as_string(time: i64) -> Option<String> {
        Some(Self::get_date(time, LONG_DATE))
    }

    /// Mirrors `getCurrentTimeSeconds()`.
    pub fn get_current_time_seconds() -> i32 {
        (Local::now().timestamp_millis() / 1000) as i32
    }
}
