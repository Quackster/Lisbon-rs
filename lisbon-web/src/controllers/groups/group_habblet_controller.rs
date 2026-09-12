//! Mirrors `org.alexdev.http.controllers.groups.GroupHabbletController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::group_member_dao::GroupMemberDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::room_dao::RoomDao;
use lisbon_server::game::groups::group_forum_type::GroupForumType;
use lisbon_server::game::groups::group_permission_type::GroupPermissionType;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::string_util::StringUtil;

use crate::dao::group_edit_dao::GroupEditDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::stickers::sticker_manager::StickerManager;
use crate::game::stickers::sticker_type::StickerType;
use crate::util::group_util::GroupUtil;
use crate::util::html_util::HtmlUtil;
use crate::util::rcon_util::RconUtil;

/// Mirrors `groupCreateForm(WebConnection)`.
pub fn group_create_form(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(web_connection.session().get_int("user.id")) else {
        return Ok(());
    };

    let group_cost = GameConfiguration::get_instance().get_integer("group.purchase.cost");

    if player_details.get_credits() < group_cost {
        let mut template = web_connection.template("groups/habblet/purchase_result_error");
        template.render_html().ok();
    } else {
        let mut template = web_connection.template("groups/habblet/group_create_form");
        template.set("groupCost", TemplateValue::of(group_cost));
        template.render_html().ok();
    }

    Ok(())
}

/// Mirrors `purchaseConfirmation(WebConnection)`.
pub fn purchase_confirmation(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let name = HtmlUtil::remove_html_tags(&StringUtil::filter_input(
        &web_connection.post().get_string("name").unwrap_or_default(),
        true,
    ));

    let mut template = web_connection.template("groups/habblet/purchase_confirmation");
    template.set("groupName", TemplateValue::of(name));
    template.set(
        "groupCost",
        TemplateValue::of(GameConfiguration::get_instance().get_integer("group.purchase.cost")),
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `purchaseAjax(WebConnection)`.
pub fn purchase_ajax(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let mut template = web_connection.template("groups/habblet/purchase_ajax");
    let Some(player_details) = PlayerDao::get_details(web_connection.session().get_int("user.id")) else {
        return Ok(());
    };

    let group_cost = GameConfiguration::get_instance().get_integer("group.purchase.cost");

    if player_details.get_credits() < group_cost {
        web_connection.send_string("");
        return Ok(());
    }

    lisbon_server::dao::mysql::currency_dao::CurrencyDao::decrease_credits(&player_details, group_cost);

    let mut parameters = HashMap::new();
    parameters.insert("userId".to_string(), player_details.get_id().to_string());
    RconUtil::send_command(RconHeader::RefreshCredits, parameters);

    let name = HtmlUtil::remove_html_tags(&StringUtil::filter_input(
        &web_connection.post().get_string("name").unwrap_or_default(),
        true,
    ));
    let _description = HtmlUtil::remove_html_tags(&StringUtil::filter_input(
        &web_connection.post().get_string("description").unwrap_or_default(),
        true,
    ));

    let group_id = GroupDao::add_group(
        &name,
        &web_connection.post().get_string("description").unwrap_or_default(),
        player_details.get_id(),
    );

    let sticker_manager = StickerManager::get_instance();

    // The Java NPE path (missing sticker) is not expressible; the lookup is
    // guarded instead.
    if let Some(sticker) = sticker_manager.get_sticker_by_data("guestbookwidget", StickerType::GroupWidget) {
        WidgetDao::purchase_widget(0, 40, 34, 6, 1, sticker.id, "", group_id, true);
    }

    if let Some(sticker) = sticker_manager.get_sticker_by_data("groupinfowidget", StickerType::GroupWidget) {
        WidgetDao::purchase_widget(0, 433, 40, 3, 1, sticker.id, "", group_id, true);
    }

    if let Some(sticker) = sticker_manager.get_sticker_by_data("memberwidget", StickerType::GroupWidget) {
        WidgetDao::purchase_widget(0, 0, 0, 0, 1, sticker.id, "", group_id, false);
    }

    if let Some(sticker) = sticker_manager.get_sticker_by_data("traxplayerwidget", StickerType::GroupWidget) {
        WidgetDao::purchase_widget(0, 0, 0, 0, 1, sticker.id, "", group_id, false);
    }

    template.set("groupName", TemplateValue::of(name));
    template.set("groupId", TemplateValue::of(group_id));
    template.set("deductedCredits", TemplateValue::of(player_details.get_credits()));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `groupSettings(WebConnection)`.
pub fn group_settings(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    if group.get_owner_id() != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    let mut template = web_connection.template("groups/habblet/group_settings");
    template.set("group", TemplateValue::of(&group));
    template.set(
        &format!("selected{}GroupType", group.get_group_type()),
        TemplateValue::of(" checked=\"checked\""),
    );
    template.set(
        &format!("selected{}ForumType", group.get_forum_type().get_id()),
        TemplateValue::of(" checked=\"checked\""),
    );
    template.set(
        &format!(
            "selected{}ForumPermissionType",
            group.get_forum_permission().get_id()
        ),
        TemplateValue::of(" checked=\"checked\""),
    );
    template.set(
        "charactersLeft",
        TemplateValue::of(255 - group.get_description().len() as i32),
    );

    let rooms = RoomDao::get_rooms_by_user_id(user_id)
        .into_iter()
        .filter(|room| {
            room.get_data().get_group_id() == 0
                || room.get_data().get_group_id() == group_id
        })
        .collect::<Vec<_>>();

    template.set("rooms", TemplateValue::of(rooms));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `checkGroupUrl(WebConnection)`.
pub fn check_group_url(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let url = web_connection.post().get_string("url").unwrap_or_default();

    let mut template = web_connection.template("groups/habblet/check_group_url");
    template.set("url", TemplateValue::of(HtmlUtil::escape(&url)));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `updateGroupSettings(WebConnection)`.
pub fn update_group_settings(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(mut group) = GroupDao::get_group(group_id) else {
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        return Ok(());
    }

    let mut name = HtmlUtil::remove_html_tags(&web_connection.post().get_string("name").unwrap_or_default());
    let mut description =
        HtmlUtil::remove_html_tags(&web_connection.post().get_string("description").unwrap_or_default());
    let mut url = web_connection
        .post()
        .get_string("url")
        .unwrap_or_default()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>();

    if url.len() > 30 {
        url = url.chars().take(30).collect();
    }

    if name.len() > 30 {
        name = name.chars().take(30).collect();
    }

    if description.len() > 255 {
        description = description.chars().take(255).collect();
    }

    let mut group_type = web_connection.post().get_int("type").unwrap_or(0);
    let mut forum_type = web_connection.post().get_int("forumType").unwrap_or(0);
    let mut forum_type_permission = web_connection
        .post()
        .get_int("newTopicPermission")
        .unwrap_or(0);
    let mut room_id = 0;

    if let Some(room_id_value) = web_connection.post().get_string("roomId") {
        if !room_id_value.is_empty() {
            room_id = room_id_value.parse().unwrap_or(0);
        }
    }

    if group_type < 0 || group_type > 3 {
        group_type = 0;
    }

    if room_id < 0 {
        room_id = 0;
    }

    if forum_type < 0 || forum_type > 1 {
        forum_type = 0;
    }

    if forum_type_permission < 0 || forum_type_permission > 2 {
        forum_type_permission = 0;
    }

    group.set_name(&name);
    group.set_description(&description);

    if group.get_group_type() != 3 {
        group.set_group_type(group_type);
    }

    group.set_forum_type(
        GroupForumType::get_by_id(forum_type).unwrap_or(GroupForumType::ALL[0]),
    );
    group.set_forum_permission(
        GroupPermissionType::get_by_id(forum_type_permission)
            .unwrap_or(GroupPermissionType::ALL[0]),
    );

    if group.get_alias().trim().is_empty() {
        if !url.trim().is_empty() {
            let existing = GroupDao::has_group_by_alias(&url);

            if !existing {
                let alias = url.clone();
                group.set_alias(&alias);
            }
        }
    }

    RoomDao::save_group_id(group.get_room_id(), 0);

    if room_id > 0 {
        let room = RoomDao::get_room_by_id(room_id);

        if room.is_none() || room.unwrap().get_data().get_owner_id() != user_id {
            room_id = 0;
        } else {
            RoomDao::save_group_id(room_id, group_id);
        }
    }

    group.set_room_id(room_id);
    group.save();

    GroupUtil::refresh_group(group_id);

    let mut template = web_connection.template("groups/habblet/update_group_settings");
    template.set("group", TemplateValue::of(&group));
    template.set("message", TemplateValue::of("Editing group settings successful"));
    template.render_html().ok();

    Ok(())
}

/// Mirrors `showBadgeEditor(WebConnection)`.
pub fn show_badge_editor(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        return Ok(());
    }

    let mut template = web_connection.template("groups/habblet/show_badge_editor");
    template.set("group", TemplateValue::of(&group));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `updateGroupBadge(WebConnection)`.
pub fn update_group_badge(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let badge = HtmlUtil::remove_html_tags(&web_connection.post().get_string("code").unwrap_or_default());

    let Some(mut group) = GroupDao::get_group(group_id) else {
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        return Ok(());
    }

    let cleaned: String = badge
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect();
    group.set_badge(&cleaned);
    group.save_badge();

    GroupUtil::refresh_group(group_id);

    web_connection.redirect(&group.generate_click_link());
    Ok(())
}

/// Mirrors `confirmDeleteGroup(WebConnection)`.
pub fn confirm_delete_group(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        return Ok(());
    }

    let mut template = web_connection.template("groups/habblet/confirm_delete_group");
    template.set("group", TemplateValue::of(&group));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `deleteGroup(WebConnection)`.
pub fn delete_group(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        return Ok(());
    }

    GroupEditDao::delete_group_widgets(group_id);
    GroupEditDao::pickup_user_widgets(group_id);

    GroupMemberDao::delete_members(group_id);
    GroupMemberDao::reset_favourites(group_id);
    GroupDao::delete(group_id);

    let mut parameters = HashMap::new();
    parameters.insert("groupId".to_string(), group_id.to_string());
    RconUtil::send_command(RconHeader::GroupDeleted, parameters);

    let mut template = web_connection.template("groups/habblet/delete_group");
    template.render_html().ok();
    Ok(())
}
