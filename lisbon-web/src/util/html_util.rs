//! Mirrors `org.alexdev.http.util.HtmlUtil`.

use std::io::{Read, Write};
use std::net::TcpStream;

use base64::Engine as _;

use lazy_static::lazy_static;
use regex::Regex;

use lisbon_server::util::config::game_configuration::GameConfiguration;

/// Mirrors `org.alexdev.http.util.HtmlUtil`.
pub struct HtmlUtil;

impl HtmlUtil {
    /// Mirrors `removeHtmlTags(String)`.
    pub fn remove_html_tags(str: &str) -> String {
        lazy_static! {
            static ref TAG_RE: Regex = Regex::new(r"<[^>]*>").unwrap();
        }
        TAG_RE.replace_all(str, "").to_string()
    }

    /// Mirrors `escape(String)`.
    pub fn escape(s: &str) -> String {
        let mut builder = String::new();
        let mut previous_was_a_space = false;

        for c in s.chars() {
            if c == ' ' {
                if previous_was_a_space {
                    builder.push_str("&nbsp;");
                    previous_was_a_space = false;
                    continue;
                }
                previous_was_a_space = true;
            } else {
                previous_was_a_space = false;
            }

            match c {
                '<' => builder.push_str("&lt;"),
                '>' => builder.push_str("&gt;"),
                '&' => builder.push_str("&amp;"),
                '"' => builder.push_str("&quot;"),
                '\n' => builder.push_str("<br>"),
                '\t' => builder.push_str("&nbsp; &nbsp; &nbsp;"),
                _ if (c as u32) < 128 => builder.push(c),
                c => builder.push_str(&format!("&#{};", c as u32)),
            }
        }
        builder
    }

    /// Mirrors `createFigureLink(String, String)`.
    pub fn create_figure_link(figure: &str, _sex: &str) -> String {
        format!(
            "{} /habbo-imaging/avatarimage?figure={figure}&size=s&direction=4&head_direction=4&crr=0&gesture=sml&frame=1",
            GameConfiguration::get_instance().get_string("site.path")
        )
    }

    /// Mirrors `encodeToString(BufferedImage, String)`.
    pub fn encode_to_string(image: &[u8], _type: &str) -> Option<String> {
        Some(base64::engine::general_purpose::STANDARD.encode(image.to_vec()))
    }

    /// Mirrors `getResponse(String)`.
    pub fn get_response(url: &str) -> Option<String> {
        let without_scheme = url.strip_prefix("http://").unwrap_or(url);
        let (authority, path) = match without_scheme.find('/') {
            Some(index) => (&without_scheme[..index], &without_scheme[index..]),
            None => (without_scheme, "/"),
        };
        let (host, port) = match authority.find(':') {
            Some(index) => {
                let port: u16 = authority[index + 1..].parse().unwrap_or(80);
                (&authority[..index], port)
            }
            None => (authority, 80u16),
        };
        let path = if path.is_empty() { "/" } else { path };

        let mut socket = TcpStream::connect((host, port)).ok()?;
        let request = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\n\r\n");
        socket.write_all(request.as_bytes()).ok()?;

        let mut buffer = Vec::new();
        socket.read_to_end(&mut buffer).ok()?;

        let body = String::from_utf8_lossy(&buffer);
        let body = body.split("\r\n\r\n").last()?.trim_end().to_string();
        Some(body)
    }
}
