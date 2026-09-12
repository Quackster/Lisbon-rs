//! Mirrors `org.alexdev.http.controllers.groups.GroupFavouriteController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::util::rcon_util::RconUtil;

/// Mirrors `confirmselectfavourite(WebConnection)`.
pub fn confirmselectfavourite(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group_name) = GroupDao::get_group_name(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let mut template = web_connection.template("groups/favourite/confirm_select_favourite");
    template.set("groupName", TemplateValue::of(group_name));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `selectfavourite(WebConnection)`.
pub fn selectfavourite(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !group.is_member(user_id) {
        web_connection.send_string("");
        return Ok(());
    }

    PlayerDao::save_favourite_group(user_id, group_id);
    let mut parameters = HashMap::new();
    parameters.insert("userId".to_string(), user_id.to_string());
    RconUtil::send_command(RconHeader::RefreshGroupPerms, parameters);

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `confirmdeselectfavourite(WebConnection)`.
pub fn confirmdeselectfavourite(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("groups/favourite/confirm_deselect_favourite");
    template.render_html().ok();
    Ok(())
}

/// Mirrors `deselectfavourite(WebConnection)`.
pub fn deselectfavourite(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !group.is_member(user_id) {
        web_connection.send_string("");
        return Ok(());
    }

    PlayerDao::save_favourite_group(user_id, 0);

    let mut parameters = HashMap::new();
    parameters.insert("userId".to_string(), user_id.to_string());
    RconUtil::send_command(RconHeader::RefreshGroupPerms, parameters);

    web_connection.send_string("OK");
    Ok(())
}
