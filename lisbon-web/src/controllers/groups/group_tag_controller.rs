//! Mirrors `org.alexdev.http.controllers.groups.GroupTagController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::tag_dao::TagDao;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::string_util::StringUtil;

use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `addGroupTag(WebConnection)`.
pub fn add_group_tag(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    let tag_list = TagDao::get_group_tags(group_id);

    if tag_list.len() as i32 >= GameConfiguration::get_instance().get_integer("max.tags.groups") {
        web_connection.send_string("taglimit");
        return Ok(());
    }

    let Some(tag) = StringUtil::is_valid_tag(
        &web_connection.post().get_string("tagName").unwrap_or_default(),
        0,
        0,
        group_id,
    ) else {
        web_connection.send_string("invalidtag");
        return Ok(());
    };

    if WordfilterManager::filter_sentence(&tag) == tag {
        StringUtil::add_tag(&tag, 0, 0, group_id);
    }

    web_connection.send_string("valid");
    Ok(())
}

/// Mirrors `removeGroupTag(WebConnection)`.
pub fn remove_group_tag(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if group.get_owner_id() != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    TagDao::remove_tag(0, 0, group_id, &web_connection.post().get_string("tagName").unwrap_or_default());
    let group_tags = TagDao::get_group_tags(group_id);

    let mut template = web_connection.template("groups/habblet/listgrouptags");
    template.set("tags", TemplateValue::of(group_tags));
    template.set("group", TemplateValue::of(&group));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `listGroupTag(WebConnection)`.
pub fn list_group_tag(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let group_tags = TagDao::get_group_tags(group_id);

    let mut template = web_connection.template("groups/habblet/listgrouptags");
    template.set("tags", TemplateValue::of(group_tags));
    template.set("group", TemplateValue::of(&group));
    template.render_html().ok();
    Ok(())
}
