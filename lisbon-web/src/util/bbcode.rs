//! Mirrors `org.alexdev.http.util.BBCode`.

use regex::Regex;

use lisbon_server::util::config::game_configuration::GameConfiguration;

/// Mirrors `org.alexdev.http.util.BBCode`.
pub struct BBCode;

impl BBCode {
    /// Mirrors `format(String, boolean)`.
    pub fn format(message: &str, allow_images: bool) -> String {
        let message = Self::parse(message, allow_images);
        GameConfiguration::get_instance().get_string("static.content.path");
        message
    }

    /// Mirrors `normalise(String)`.
    pub fn normalise(message: &str) -> String {
        let message = message.replace('\r', "\n");
        let message = message.replace("[/quote]\n\n", "[/quote]");
        let message = message.replace("[/quote]\n", "[/quote]");
        message.replace('\n', "[br]")
    }

    /// Mirrors `parse(String, boolean)`.
    fn parse(message: &str, allow_images: bool) -> String {
        let site_path = GameConfiguration::get_instance().get_string("site.path");
        let mut message = message.to_string();

        if message.contains("javascript:") || message.contains("document.write") {
            message = message.replace("javascript:", "");
            message = message.replace("document.write", "");
        }

        for (pattern, replacement) in [
            (r"\[b](.*?)\[/b]", "<b>$1</b>"),
            (r"\[i](.*?)\[/i]", "<i>$1</i>"),
            (r"\[u](.*?)\[/u]", "<u>$1</u>"),
            (r"\[s](.*?)\[/s]", "<s>$1</s>"),
            (r"\[strike](.*?)\[/strike]", "<strike>$1</strike>"),
            (r"\[link=(.*?)](.*?)\[/link]", r#"<a href="$1">$2</a>"#),
            (r"\[url=(.*?)](.*?)\[/url]", r#"<a href="$1">$2</a>"#),
            (
                r"\[color=(orange|red|yellow|green|cyan|blue|gray|black|white)](.*?)\[/color]",
                r#"<font color="$1">$2</font>"#,
            ),
            (
                r"\[color=(#[0-9a-fA-F]{6})](.*?)\[/color]",
                r#"<font color="$1">$2</font>"#,
            ),
            (
                r"\[size=small](.*?)\[/size]",
                r#"<span style="font-size: 9px;">$1</span>"#,
            ),
            (
                r"\[size=large](.*?)\[/size]",
                r#"<span style="font-size: 14px;">$1</span>"#,
            ),
            (r"\[code](.*?)\[/code]", "<pre>$1</pre>"),
        ] {
            message = Regex::new(pattern)
                .unwrap()
                .replace_all(&message, replacement)
                .to_string();
        }

        message = Regex::new(r"\[habbo=(.*?)](.*?)\[/habbo]")
            .unwrap()
            .replace_all(&message, |caps: &regex::Captures| {
                format!(
                    "<a href=\"{} /home/{}/id\">{}</a>",
                    site_path,
                    caps.get(1).map(|m| m.as_str()).unwrap_or(""),
                    caps.get(2).map(|m| m.as_str()).unwrap_or("")
                )
            })
            .to_string();

        message = Regex::new(r"\[room=(.*?)](.*?)\[/room]")
            .unwrap()
            .replace_all(&message, |caps: &regex::Captures| {
                let room_id = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                format!(
                    "<a onclick=\"roomForward(this, '{room_id}', 'private'); return false;\" target=\"client\" href=\"{site_path} /client?forwardId=2&roomId={room_id}\">{}</a>",
                    caps.get(2).map(|m| m.as_str()).unwrap_or("")
                )
            })
            .to_string();

        message = Regex::new(r"\[group=(.*?)](.*?)\[/group]")
            .unwrap()
            .replace_all(&message, |caps: &regex::Captures| {
                format!(
                    "<a href=\"{} /groups/{}/id\">{}</a>",
                    site_path,
                    caps.get(1).map(|m| m.as_str()).unwrap_or(""),
                    caps.get(2).map(|m| m.as_str()).unwrap_or("")
                )
            })
            .to_string();

        for _ in 0..10 {
            message = Regex::new(r"\[quote](.*?)\[/quote]")
                .unwrap()
                .replace_all(&message, |caps: &regex::Captures| {
                    format!(
                        "<div class=\"bbcode-quote\">{}</div>",
                        caps.get(1).map(|m| m.as_str()).unwrap_or("")
                    )
                })
                .to_string();
        }

        message = message.replace("[br]", "<br>");

        if allow_images {
            message = Regex::new(r"\[img=(.*?)](.*?)\[/img]")
                .unwrap()
                .replace_all(&message, |caps: &regex::Captures| {
                    format!(
                        "<img alt=\"{}\" src=\"{}\"/>",
                        caps.get(1).map(|m| m.as_str()).unwrap_or(""),
                        caps.get(2).map(|m| m.as_str()).unwrap_or("")
                    )
                })
                .to_string();
            message = Regex::new(r"\[img](.*?)\[/img]")
                .unwrap()
                .replace_all(&message, |caps: &regex::Captures| {
                    format!(
                        "<img src=\"{}\"/>",
                        caps.get(1).map(|m| m.as_str()).unwrap_or("")
                    )
                })
                .to_string();
            message = Regex::new(r"\[img height='(.*?)' width='(.*?)'](.*?)\[/img]")
                .unwrap()
                .replace_all(&message, |caps: &regex::Captures| {
                    format!(
                        "<img height=\"{}\" width=\"{}\" src=\"{}\"/>",
                        caps.get(1).map(|m| m.as_str()).unwrap_or(""),
                        caps.get(2).map(|m| m.as_str()).unwrap_or(""),
                        caps.get(3).map(|m| m.as_str()).unwrap_or("")
                    )
                })
                .to_string();
            message = Regex::new(r"\[img](.*?)\[/img]")
                .unwrap()
                .replace_all(&message, |caps: &regex::Captures| {
                    format!(
                        "<img src=\"{}\"/>",
                        caps.get(1).map(|m| m.as_str()).unwrap_or("")
                    )
                })
                .to_string();
            message = Regex::new(r"\[article_images](.*?)\[/article_images]")
                .unwrap()
                .replace_all(&message, r#"<div class="article-images clearfix">$1</div>"#)
                .to_string();
            message = Regex::new(r"\[article_image](.*?)\[/article_image]")
                .unwrap()
                .replace_all(
                    &message,
                    r#"<a href="$1" style="background-image: url($1); background-position: -0px -0px"></a>"#,
                )
                .to_string();
            message = Regex::new(r"\[article_image x=(.*?) y=(.*?)](.*?)\[/article_image]")
                .unwrap()
                .replace_all(
                    &message,
                    r#"<a href="$3" style="background-image: url($3); background-position: $1 $2"></a>"#,
                )
                .to_string();
            message = Regex::new(r"\[center](.*?)\[/center]")
                .unwrap()
                .replace_all(&message, "<center>$1</center>")
                .to_string();
            message = message.replacen("<br><br>", "<br>", 1);
        }

        message
    }
}
