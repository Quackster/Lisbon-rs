//! Mirrors `net.h4bbo.lisbon.util.StringUtil`.

use std::collections::HashMap;

use md5::{Digest, Md5};
use unicode_normalization::UnicodeNormalization;

use crate::dao::mysql::tag_dao::TagDao;

pub struct StringUtil;

impl StringUtil {
    /// Mirrors `isNullOrEmpty`.
    pub fn is_null_or_empty(param: Option<&str>) -> bool {
        match param {
            None => true,
            Some(s) => s.trim().is_empty(),
        }
    }

    /// Mirrors `filterInput`.
    pub fn filter_input(input: &str, filter_newline: bool) -> String {
        let mut out = input
            .replace('\u{1}', " ")
            .replace('\u{2}', " ")
            .replace('\t', " ")
            .replace('\n', " ")
            .replace('\u{c}', " ");

        if filter_newline {
            out = out.replace('\r', " ");
        }

        if crate::util::config::game_configuration::GameConfiguration::get_instance()
            .get_bool("normalise.input.strings")
        {
            // Mirrors `Normalizer.normalize(input, NFD)`.
            out = out.chars().nfd().collect();
        }

        if out.contains('∂') && out.contains('∫') && out.contains('å') && out.contains('æ') {
            out = out
                .replace('∂', "")
                .replace('∫', "")
                .replace('å', "")
                .replace('æ', "");
        }

        out
    }

    /// Mirrors `paginate` (2-arg overload).
    pub fn paginate<T>(original_list: &[T], chunk_size: usize) -> HashMap<usize, Vec<T>>
    where
        T: Clone,
    {
        Self::paginate_impl(original_list, chunk_size, false)
    }

    /// Mirrors `paginate` (3-arg overload); the 2-arg version cannot be overloaded
    /// in Rust, so it is exposed under a distinct name.
    pub fn paginate_ext<T>(original_list: &[T], chunk_size: usize, empty_first_page: bool) -> HashMap<usize, Vec<T>>
    where
        T: Clone,
    {
        Self::paginate_impl(original_list, chunk_size, empty_first_page)
    }

    fn paginate_impl<T>(original_list: &[T], chunk_size: usize, empty_first_page: bool) -> HashMap<usize, Vec<T>>
    where
        T: Clone,
    {
        let mut chunks: HashMap<usize, Vec<T>> = HashMap::new();
        if chunk_size == 0 {
            return chunks;
        }

        let mut i = 0usize;
        while i < original_list.len() / chunk_size {
            let from = i * chunk_size;
            let to = from + chunk_size;
            chunks.insert(i, original_list[from..to].to_vec());
            i += 1;
        }

        if original_list.len() % chunk_size != 0 {
            let from = original_list.len() - original_list.len() % chunk_size;
            chunks.insert(i, original_list[from..].to_vec());
        }

        if empty_first_page && chunks.is_empty() {
            chunks.insert(0, vec![]);
        }

        chunks
    }

    /// Mirrors `format` (round to 2 decimal places).
    pub fn format(decimal: f64) -> f64 {
        (decimal * 100.0).round() / 100.0
    }

    /// Mirrors `split`.
    pub fn split(str: &str, delim: &str) -> Vec<String> {
        str.split(delim).map(|s| s.to_string()).collect()
    }

    /// Mirrors `getWords`.
    pub fn get_words(s: &str) -> Vec<String> {
        s.split_whitespace()
            .map(|w| w.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>())
            .collect()
    }

    /// Mirrors `getCharset` — Rust strings are always UTF-8.
    pub fn get_charset() -> &'static str {
        "UTF-8"
    }

    /// Mirrors `toAlphabetic`.
    pub fn to_alphabetic(i: i32) -> String {
        let i = i - 1;
        if i < 0 {
            return format!("-{}", Self::to_alphabetic(-i - 1));
        }

        let quot = i / 26;
        let rem = (i % 26) as u32;
        let letter = (b'A' + rem as u8) as char;
        if quot == 0 {
            letter.to_string()
        } else {
            format!("{}{}", Self::to_alphabetic(quot - 1), letter)
        }
    }

    /// Mirrors `hasValue`.
    pub fn has_value(first_list: &[String], second_list: &[String]) -> bool {
        first_list.iter().any(|str| second_list.contains(str))
    }

    /// Mirrors `md5`.
    pub fn md5(input: &str) -> String {
        let mut hasher = Md5::default();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Mirrors `isValidTag`.
    pub fn is_valid_tag(tag: &str, user_id: i32, room_id: i32, group_id: i32) -> Option<String> {
        let format_tag =
            Self::normalize_space(Self::filter_input(tag, false).as_str()).replace(',', "");
        let format_tag = regex::Regex::new(r"<[^>]*>")
            .unwrap()
            .replace(&format_tag, "")
            .to_lowercase();

        if tag.chars().count() <= 1
            || tag.trim().is_empty()
            || tag.chars().count() > 20
            || TagDao::has_tag(user_id, room_id, group_id, tag)
        {
            return None;
        }

        Some(format_tag)
    }

    /// Mirrors `StringUtils.normalizeSpace`.
    fn normalize_space(s: &str) -> String {
        s.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// Mirrors `addTag`.
    pub fn add_tag(tag: &str, user_id: i32, room_id: i32, group_id: i32) {
        let (tag, check_again) = if tag.eq_ignore_ascii_case("br") || tag.eq_ignore_ascii_case("brasil") {
            ("brazil", true)
        } else if tag.eq_ignore_ascii_case("spanish") || tag.eq_ignore_ascii_case("es") {
            ("español", true)
        } else {
            (tag, false)
        };

        if check_again && TagDao::has_tag(user_id, room_id, group_id, tag) {
            return;
        }

        TagDao::add_tag(user_id, room_id, group_id, tag);
    }

    /// Mirrors `replaceAlertMessage` (the Java `Player` argument is reduced to
    /// its name; the flash-connection branch is commented out in Java).
    pub fn replace_alert_message(message: &str, player_name: &str) -> String {
        message
            .replace("\r\n", "<br>")
            .replace('\r', "<br>")
            .replace('\n', "<br>")
            .replace("%username%", player_name)
    }
}
