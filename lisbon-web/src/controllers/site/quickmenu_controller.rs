//! Mirrors `org.alexdev.http.controllers.site.QuickmenuController`.

use std::cmp::Reverse;

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::room_dao::RoomDao;


use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `groups(WebConnection)`.
pub fn groups(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let groups = GroupDao::get_joined_groups(web_connection.session().get_int("user.id"));

    let mut tpl = web_connection.template("quickmenu/groups");
    tpl.set("groups", TemplateValue::of(&groups));
    tpl.render();

    Ok(())
}

/// Mirrors `friends(WebConnection)`.
pub fn friends(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let friends = MessengerDao::get_friends(web_connection.session().get_int("user.id"));

    let mut friends_online: Vec<_> = friends.values().filter(|friend| friend.is_online()).collect();
    friends_online.sort_by_key(|friend| Reverse(friend.get_last_online()));
    friends_online.truncate(10);

    let mut friends_offline: Vec<_> = friends
        .values()
        .filter(|friend| !friend.is_online())
        .collect();
    friends_offline.sort_by_key(|friend| Reverse(friend.get_last_online()));
    friends_offline.truncate(10);

    let mut tpl = web_connection.template("quickmenu/friends_all");
    tpl.set("onlineFriends", TemplateValue::of(&friends_online));
    tpl.set("offlineFriends", TemplateValue::of(&friends_offline));
    tpl.render();

    Ok(())
}

/// Mirrors `rooms(WebConnection)`.
pub fn rooms(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut tpl = web_connection.template("quickmenu/rooms");
    let rooms = RoomDao::get_rooms_by_user_id(web_connection.session().get_int("user.id"));
    tpl.set("rooms", TemplateValue::of(&rooms));
    tpl.render();

    Ok(())
}
