//! Mirrors `org.alexdev.http.controllers.habblet.XmlController`.

use lisbon_server::dao::mysql::badge_dao::BadgeDao;
use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::string_util::StringUtil;

use crate::duckhttpd::{ResponseBuilder, WebConnection};

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Mirrors `promoHabbos(WebConnection)`.
pub fn promo_habbos(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let site_path = GameConfiguration::get_instance().get_string("site.path");
    let mut entries = String::new();

    for h in PlayerDao::get_recent_habbos(10, false) {
        let mut badge_url = String::new();

        let mut badges = BadgeDao::get_badges(h.get_id());
        badges.sort_by_key(|badge| badge.get_slot_id());

        if !badges.is_empty() {
            let first = badges.first().unwrap();
            badge_url = format!(
                "{}/c_images/Badges/{}.gif",
                GameConfiguration::get_instance().get_string("static.content.path"),
                first.get_badge_code(),
            );
        }

        let mut group_badge = String::new();

        if h.get_favourite_group_id() > 0 {
            if let Some(group) = GroupDao::get_group(h.get_favourite_group_id()) {
                group_badge = format!(
                    "{site_path}/habbo-imaging/badge/{}.gif",
                    group.get_badge()
                );
            }
        }

        entries.push_str(&format!(
            "<habbo id=\"{}\" name=\"{}\" motto=\"{}\" url=\"/home/{}\" image=\"{site_path}/habbo-imaging/avatarimage?figure={}&size=b&direction=4&head_direction=5&crr=0&gesture=&frame=1\" badge=\"{}\" status=\"{}\" groupBadge=\"{}\"/>",
            h.get_id(),
            xml_escape(h.get_name()),
            xml_escape(h.get_motto()),
            xml_escape(h.get_name()),
            xml_escape(h.get_figure()),
            xml_escape(&badge_url),
            if h.is_online() { "1" } else { "0" },
            xml_escape(&group_badge),
        ));
    }

    let response = ResponseBuilder::create_with_content_type("text/xml", format!("<habbos>{entries}</habbos>"));
    web_connection.send(response);
    Ok(())
}

/// Mirrors `promoHabbosV2(WebConnection)`.
pub fn promo_habbos_v2(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let site_path = GameConfiguration::get_instance().get_string("site.path");
    let mut entries = String::new();

    for h in PlayerDao::get_recent_habbos(30, true) {
        entries.push_str(&format!(
            "<habbo gender=\"{}\" figure=\"{}\" hash=\"{}\"/>",
            h.get_sex().to_lowercase(),
            xml_escape(h.get_figure()),
            StringUtil::md5(h.get_figure()),
        ));
    }

    let response = ResponseBuilder::create_with_content_type(
        "text/xml",
        format!("<habbos url=\"{site_path}/habbo-imaging/avatar/\">{entries}</habbos>"),
    );
    web_connection.send(response);
    Ok(())
}
