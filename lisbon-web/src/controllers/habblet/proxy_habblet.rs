//! Mirrors `org.alexdev.http.controllers.habblet.ProxyHabblet`.

use std::collections::HashMap;

use rand::Rng;

use lisbon_server::dao::mysql::room_dao::RoomDao;
use lisbon_server::dao::storage::Storage;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::controllers::site::minimail_controller::append_messages;
use crate::dao::community_dao::CommunityDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::server::watchdog::TAG_CLOUD_20;
use crate::util::rcon_util::RconUtil;
use crate::util::xss_util::XssUtil;

/// Mirrors `moreInfo(WebConnection)`.
pub fn more_info(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.get().contains("hid") {
        web_connection.send_string("");
        return Ok(());
    }

    let hid = web_connection.get().get_string("hid").unwrap_or_default();

    if hid == "h21" {
        web_connection.send_string(concat!(
            "\n",
            "<div id=\"staffpicks-rooms-habblet-list-container\" class=\"habblet-list-container groups-list\">\n",
            "    <ul class=\"habblet-list\">\n",
            "\n",
            "        <li class=\"even room-occupancy-2\" roomid=\"1\">\n",
            "            <div>\n",
            "                <span class=\"room-name\"><a href=\"http://localhost/client?forwardId=2&amp;roomId=1\" onclick=\"HabboClient.roomForward(this, '1', 'private'); return false;\" target=\"client\">Room name</a></span>\n",
            "                <span class=\"room-owner\"><a href=\"http://localhost/home/Alex\">Alex</a></span>                \n",
            "\t\t\t\t<p>test</p>\n",
            "            </div>\n",
            "        </li>\n",
            "    </ul>\n",
            "</div>\n",
            "\n"
        ));
        return Ok(());
    }

    if hid == "h122" {
        let limit = GameConfiguration::get_instance().get_integer("hot.groups.community.limit");

        let mut _hot_groups = CommunityDao::get_hot_groups(limit, 0);
        _hot_groups.sort_by(|a, b| b.1.cmp(&a.1));

        let mut _hot_hidden_groups = CommunityDao::get_hot_groups(limit, limit);
        _hot_hidden_groups.sort_by(|a, b| b.1.cmp(&a.1));

        let mut template = web_connection.template("habblet/community_hot_groups");
        template.set(
            "hotGroups",
            TemplateValue::of(
                _hot_groups
                    .iter()
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<_>>(),
            ),
        );
        template.set(
            "hotHiddenGroups",
            TemplateValue::of(
                _hot_hidden_groups
                    .iter()
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<_>>(),
            ),
        );
        template.render();

        return Ok(());
    }

    if hid == "h120" {
        let mut template = web_connection.template("habblet/showMoreRooms");
        template.set("highestRatedRooms", TemplateValue::of(RoomDao::get_highest_rated_rooms(5, 0)));
        template.set(
            "highestHiddenRatedRooms",
            TemplateValue::of(RoomDao::get_highest_rated_rooms(5, 5)),
        );
        template.render();
        return Ok(());
    }

    if hid == "h24" {
        let mut template = web_connection.template("habblet/tagList");
        template.set("tagCloud", TemplateValue::of(TAG_CLOUD_20.read().clone()));
        template.render();
        return Ok(());
    }

    if hid == "groups" {
        let mut _hot_groups = CommunityDao::get_hot_groups(
            GameConfiguration::get_instance().get_integer("hot.groups.limit"),
            0,
        );
        _hot_groups.sort_by(|a, b| b.1.cmp(&a.1));

        let mut template = web_connection.template("habblet/hot_groups");
        template.set(
            "groups",
            TemplateValue::of(
                _hot_groups
                    .iter()
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<_>>(),
            ),
        );
        template.render();
        return Ok(());
    }

    web_connection.send_string("");
    Ok(())
}

/// Mirrors `minimail(WebConnection)`.
pub fn minimail(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let habblet_key = web_connection
        .get()
        .get_string("habbletKey")
        .unwrap_or_default();

    if habblet_key.eq_ignore_ascii_case("news") {
        web_connection.send_string(concat!(
            "<div class=\"habblet-container \">\t\t\n",
            "\t\n",
            "\t<div id=\"news-habblet-container\">\n",
            "\t\n",
            "\t\t<div class=\"title\">\n",
            "\t\t\n",
            "\t\t\t<div class=\"habblet-close\"></div>\n",
            "\t\t\t\n",
            "\t\t\t<div>The shit you don't even wanna know!</div>\n",
            "\t\t\t\n",
            "\t\t</div>\n",
            "\t\t\n",
            "\t\t<div class=\"content-container\">\n",
            "\t\t\n",
            "\t\t\t<div id=\"news-articles\">\n",
            "\t\t\t\n",
            "\t\t\t\t<ul id=\"news-articlelist\" class=\"articlelist\" style=\"display: none\">\n",
            "\n",
            "\t\t\t\t</ul>\n",
            "\t\t\t\t\n",
            "\t\t\t</div>\n",
            "\t\t\t\n",
            "\t\t</div>\n",
            "\t\t\n",
            "\t\t<div class=\"news-footer\"></div>\n",
            "\t\n",
            "\t</div>\n",
            "\n",
            "\t<script type=\"text/javascript\">    \n",
            "\t\tL10N.put(\"news.promo.readmore\", \"Read more\").put(\"news.promo.close\", \"Close article\");\n",
            "\t\tNews.init(false);\n",
            "\t</script>\n",
            "\n",
            "</div>\n",
            "\n",
            "<!-- dependencies\n",
            "<link rel=\"stylesheet\" href=\"http://images.habbo.com/habboweb/%web_build%/web-gallery/v2/styles/news.css\" type=\"text/css\" />\n",
            "<script src=\"http://images.habbo.com/habboweb/%web_build%/web-gallery/static/js/news.js\" type=\"text/javascript\"></script>\n",
            "-->"
        ));
        return Ok(());
    }

    let mut template = web_connection.template("habblet/minimail");
    web_connection
        .session()
        .set("minimailLabel", SessionValue::Str("inbox".to_string()));
    append_messages(
        web_connection,
        &mut template,
        true,
        false,
        false,
        false,
        false,
        false,
    );
    template.set("minimailClient", TemplateValue::of(true));
    template.render();
    Ok(())
}

/// Mirrors `clearHand(WebConnection)`.
pub fn clearhand(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.session().get_boolean("authenticated") {
        connection.send_string("");
        return Ok(());
    }

    if !XssUtil::verify_key(connection, "/credits") {
        connection.send_string("Failed to securely verify request");
        return Ok(());
    }

    let user_id = connection.session().get_int("user.id");

    // Mirrors `ItemDao.deleteHandItems(int)`.
    Storage::get_storage().execute(&format!(
        "DELETE FROM items WHERE is_hidden = 0 AND is_trading = 0 AND room_id = 0 AND user_id = {user_id}"
    ));

    RconUtil::send_command(
        RconHeader::RefreshHand,
        HashMap::from([("userId".to_string(), user_id.to_string())]),
    );

    connection.send_string("");
    Ok(())
}

/// Mirrors `tokenGenerate(WebConnection)`.
pub fn token_generate(connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !connection.session().get_boolean("authenticated") {
        connection.send_string("");
        return Ok(());
    }

    let mut bytes = [0u8; 16];
    rand::thread_rng().fill(&mut bytes);
    let uuid = format!("token-{}", hex::encode(bytes));
    connection
        .session()
        .set("authenticationToken", SessionValue::Str(uuid.clone()));
    connection.send_string(&uuid);
    Ok(())
}
