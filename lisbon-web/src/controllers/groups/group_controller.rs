//! Mirrors `org.alexdev.http.controllers.groups.GroupController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::room_dao::RoomDao;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::group_edit_dao::GroupEditDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{Settings, TemplateValue, WebConnection};
use http::StatusCode;
use crate::util::home_util::HomeUtil;
use crate::util::xss_util::XssUtil;

fn not_found(web_connection: &WebConnection) -> bool {
    let Some(response) = Settings::get_instance()
        .get_default_responses()
        .get_response(StatusCode::NOT_FOUND, web_connection)
    else {
        return false;
    };

    web_connection.send(response);
    true
}

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

/// Mirrors `viewGroup(WebConnection)`.
pub fn view_group(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    if !web_connection.session().contains("authenticated") {
        return Ok(());
    }

    web_connection.session().set("page", SessionValue::Str("community".into()));

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    let group = if is_numeric(match0) && web_connection.get_route_request().ends_with("/id") {
        let Some(found) = GroupDao::get_group(match0.parse().unwrap_or(0)) else {
            not_found(web_connection);
            return Ok(());
        };

        if !found.get_alias().trim().is_empty() {
            web_connection.redirect(&format!("/groups/{}", found.get_alias()));
            return Ok(());
        }

        found
    } else if !web_connection.get_route_request().ends_with("/id") {
        match GroupDao::get_group_by_alias(match0) {
            Some(group) => group,
            None => {
                not_found(web_connection);
                return Ok(());
            }
        }
    } else {
        not_found(web_connection);
        return Ok(());
    };

    let group_id = group.get_id();
    let mut session_time = -1;

    if web_connection.session().get_boolean("authenticated") {
        let user_id = web_connection.session().get_int("user.id");
        session_time = GroupEditDao::get_session(user_id, group_id);
    }

    if session_time != -1 {
        web_connection.session().delete("homeEditSession");
        web_connection.session().set("groupEditSession", SessionValue::Int(group_id));
    }

    let alias = group.get_alias();

    if alias.eq_ignore_ascii_case("battleball_rebound") {
        web_connection.session().set("page", SessionValue::Str("games".into()));
    }

    if alias.eq_ignore_ascii_case("lido") {
        web_connection.session().set("page", SessionValue::Str("games".into()));
    }

    if alias.eq_ignore_ascii_case("snow_storm") {
        web_connection.session().set("page", SessionValue::Str("games".into()));
    }

    if alias.eq_ignore_ascii_case("wobble_squabble") {
        web_connection.session().set("page", SessionValue::Str("games".into()));
    }

    let mut template = web_connection.template("groups");
    template.set("editMode", TemplateValue::of(session_time != -1));
    template.set("group", TemplateValue::of(&group));
    template.set(
        "stickers",
        TemplateValue::of(WidgetDao::get_group_widgets_by_placement(group_id, true)),
    );
    template.set("tags", TemplateValue::of(lisbon_server::dao::mysql::tag_dao::TagDao::get_group_tags(group_id)));

    let group_widgets = WidgetDao::get_group_widgets(group_id);
    let guestbook = group_widgets.iter().find(|widget| {
        widget
            .get_product()
            .map(|product| product.data.eq_ignore_ascii_case("guestbookwidget"))
            .unwrap_or(false)
    });

    if let Some(guestbook) = guestbook {
        template.set("guestbookSetting", TemplateValue::of(guestbook.get_guestbook_state()));
    }

    template.set("stickerLimit", TemplateValue::of(HomeUtil::get_sticker_limit(true)));

    if session_time != -1 {
        template.set(
            "expireMinutes",
            TemplateValue::of((session_time - DateUtil::get_current_time_seconds() as i64) / 60),
        );
    }

    if group.get_room_id() > 0 {
        if let Some(room) = RoomDao::get_room_by_id(group.get_room_id()) {
            template.set("room", TemplateValue::of(&room));
        }
    }

    template.set("hasMember", TemplateValue::of(false));

    if web_connection.session().get_boolean("authenticated") {
        let user_id = web_connection.session().get_int("user.id");

        if let Some(group_member) = group.get_member(user_id) {
            template.set("hasMember", TemplateValue::of(true));
            template.set("groupMember", TemplateValue::of(group_member));
        }
    }

    template.render_html().ok();
    Ok(())
}

/// Mirrors `startEditingSession(WebConnection)`.
pub fn start_editing_session(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    let group = if is_numeric(match0) {
        GroupDao::get_group(match0.parse().unwrap_or(0))
    } else {
        None
    };

    if group.is_none() {
        not_found(web_connection);
        return Ok(());
    }

    if let Some(group) = group {
        if group.is_member(user_id) && group.has_administrator(user_id) {
            if !GroupEditDao::has_session(user_id, group.get_id()) {
                GroupEditDao::delete(user_id, group.get_id());

                GroupEditDao::create_session(user_id, group.get_id());
                web_connection.session().delete("homeEditSession");
                web_connection.session()
                    .set("groupEditSession", SessionValue::Int(group.get_id()));
            }
        }

        web_connection.redirect(&group.generate_click_link());
    }

    Ok(())
}

/// Mirrors `cancelEditingSession(WebConnection)`.
pub fn cancel_editing_session(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    if !web_connection.session().contains("groupEditSession") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.session().get_int("groupEditSession");

    if GroupEditDao::has_session(user_id, group_id) {
        GroupEditDao::delete(user_id, group_id);
        web_connection.session().delete("homeEditSession");
        web_connection.session().delete("groupEditSession");
    }

    if let Some(group) = GroupDao::get_group(group_id) {
        web_connection.redirect(&group.generate_click_link());
    }

    Ok(())
}

/// Mirrors `saveEditingSession(WebConnection)`.
pub fn save_editing_session(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(web_connection.session().get_int("user.id")) else {
        web_connection.session().delete("user.id");
        web_connection.session().delete("authenticated");
        web_connection.redirect("/");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");
    let group_id = web_connection.session().get_int("groupEditSession");

    let Some(mut group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !GroupEditDao::has_session(user_id, group.get_id()) {
        web_connection.send_string("");
        return Ok(());
    }

    let group_widgets = WidgetDao::get_group_widgets_by_placement(group_id, true);

    if web_connection.post().contains("background") {
        let background_value = web_connection.post().get_string("background").unwrap_or_default();
        let background_id = background_value
            .split(':')
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        let widget = WidgetDao::get_inventory_widgets(player_details.get_id())
            .into_iter()
            .find(|widget| widget.id == background_id);

        if let Some(widget) = &widget {
            if let Some(product) = widget.get_product() {
                let data = product.data.clone();
                group.set_background(&data);
                group.save_background();
                group.save();
            }
        }
    }

    for field in ["stickers", "widgets", "stickienotes"] {
        if !web_connection.post().contains(field) {
            continue;
        }

        let value = web_connection.post().get_string(field).unwrap_or_default();
        let sticker_data: Vec<&str> = value.split('/').collect();

        if field == "stickers" && (sticker_data.len() as i32) >= HomeUtil::get_sticker_limit(true) {
            web_connection.send_string("");
            return Ok(());
        }

        for sticker in sticker_data {
            let sticker_id = sticker
                .split(':')
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap_or(0);
            let stripped = sticker.replace(format!("{sticker_id}:").as_str(), "");
            let coord_data: Vec<&str> = stripped.split(',').collect();

            let x = coord_data
                .first()
                .and_then(|part| part.parse().ok())
                .unwrap_or(0);
            let y = coord_data.get(1).and_then(|part| part.parse().ok()).unwrap_or(0);
            let z = coord_data.get(2).and_then(|part| part.parse().ok()).unwrap_or(0);

            let mut widget = group_widgets.iter().find(|widget| widget.id == sticker_id).cloned();

            if let Some(widget) = &mut widget {
                widget.set_x(x);
                widget.set_y(y);
                widget.set_z(z);
                widget.save();
            }
        }
    }

    GroupEditDao::delete(user_id, group_id);

    web_connection.session().delete("homeEditSession");
    web_connection.session().delete("groupEditSession");

    web_connection.send_string(&format!(
        "<script language=\"JavaScript\" type=\"text/javascript\">\nwaitAndGo('{}');\n</script>",
        group.generate_click_link(),
    ));
    Ok(())
}

/// Mirrors `groupinfo(WebConnection)`.
pub fn groupinfo(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let mut template = web_connection.template("homes/widget/habblet/groupinfo");
    template.set("group", TemplateValue::of(&group));
    template.render_html().ok();
    Ok(())
}
