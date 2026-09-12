//! Mirrors `org.alexdev.http.controllers.homes.NoteEditorController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::room_dao::RoomDao;

use crate::dao::group_edit_dao::GroupEditDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::stickers::sticker_manager::StickerManager;
use crate::game::stickers::sticker_type::StickerType;
use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

fn truncate(value: String, max: usize) -> String {
    if value.chars().count() > max {
        value.chars().take(max).collect()
    } else {
        value
    }
}

/// Mirrors `noteEditor(WebConnection)`.
pub fn note_editor(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut note_text = web_connection.post().get_string("noteText").unwrap_or_default();
    let skin = web_connection.post().get_int("skin").unwrap_or(0);

    if note_text.chars().count() > 500 {
        note_text = truncate(note_text, 500);
    }

    let mut template = web_connection.template("homes/editor/noteeditor");

    if skin > 0 && skin < 9 {
        template.set(
            &format!("skin{skin}Selected"),
            TemplateValue::of(" selected"),
        );
    }

    template.set("noteText", TemplateValue::of(note_text));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `notePreview(WebConnection)`.
pub fn note_preview(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut note_text = BBCode::format(
        &HtmlUtil::escape(&BBCode::normalise(
            &web_connection.post().get_string("noteText").unwrap_or_default(),
        )),
        false,
    );
    let skin = web_connection.post().get_int("skin").unwrap_or(0);

    if note_text.chars().count() > 500 {
        note_text = truncate(note_text, 500);
    }

    let mut template = web_connection.template("homes/editor/preview");
    template.set(
        "skin",
        TemplateValue::of(StickerManager::get_instance().get_skin(skin)),
    );
    template.set("noteText", TemplateValue::of(note_text));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `search(WebConnection)`.
pub fn search(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let query = web_connection.get().get_string("query").unwrap_or_default();
    let scope = web_connection.get().get_int("scope").unwrap_or(0);

    let mut query_search: Vec<(String, String)> = Vec::new();
    let type_ = match scope {
        1 => "habbo",
        2 => "room",
        _ => "group",
    };

    match scope {
        1 => {
            let mut searched_friends: Vec<_> = Vec::new();

            for player_id in MessengerDao::search(&query) {
                if let Some(player_details) = PlayerDao::get_details(player_id) {
                    searched_friends.push(player_details);
                }
            }

            searched_friends.sort_by(|a, b| a.get_name().cmp(b.get_name()));

            for player_details in searched_friends.iter().take(10) {
                query_search.push((
                    player_details.get_name().to_string(),
                    player_details.get_id().to_string(),
                ));
            }
        }
        2 => {
            for room in RoomDao::search_rooms(&query, -1, 30).iter().take(10) {
                query_search.push((
                    room.get_data().get_name().to_string(),
                    room.get_data().get_id().to_string(),
                ));
            }
        }
        _ => {
            for group in GroupDao::query_search(&query).iter().take(10) {
                query_search.push((
                    group.get_name().to_string(),
                    group.get_id().to_string(),
                ));
            }
        }
    }

    let mut tpl = web_connection.template("homes/editor/search");
    tpl.set(
        "querySearch",
        TemplateValue::json(
            serde_json::Value::Array(
                query_search
                    .iter()
                    .map(|(name, id)| serde_json::json!([name, id]))
                    .collect::<Vec<_>>(),
            ),
        ),
    );
    tpl.set("type", TemplateValue::of(type_));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `place(WebConnection)`.
pub fn place(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let skin = web_connection.post().get_int("skin").unwrap_or(0);
    let mut note_text = web_connection.post().get_string("noteText").unwrap_or_default();

    if note_text.chars().count() > 500 {
        note_text = truncate(note_text, 500);
    }

    let is_group_edit = web_connection.session().contains("groupEditSession");

    if is_group_edit {
        let group_id = web_connection.session().get_int("groupEditSession");

        let Some(group) = GroupDao::get_group(group_id) else {
            web_connection.send_string("");
            return Ok(());
        };

        if !GroupEditDao::has_session(user_id, group.get_id()) {
            web_connection.send_string("");
            return Ok(());
        }
    } else {
        if !web_connection.session().contains("homeEditSession") {
            web_connection.send_string("");
            return Ok(());
        }
    }

    let mut widget = match WidgetDao::get_inventory_widgets_by_type(user_id, StickerType::Note.type_id())
        .into_iter()
        .next()
    {
        Some(widget) => widget,
        None => {
            // Java throws `IndexOutOfBoundsException` on an empty list.
            return Ok(());
        }
    };

    widget.set_x(20);
    widget.set_y(30);
    widget.set_z(1);

    if is_group_edit {
        widget.set_group_id(web_connection.session().get_int("groupEditSession"));
    }

    widget.set_text(&note_text);
    widget.set_skin_id(skin);
    widget.set_placed(true);
    widget.save();

    web_connection.set_header("X-JSON", &widget.id.to_string());

    let mut tpl = widget.template(web_connection);
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `stickieEdit(WebConnection)`.
pub fn stickie_edit(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let is_group_edit = web_connection.session().contains("groupEditSession");

    if is_group_edit {
        let group_id = web_connection.session().get_int("groupEditSession");

        let Some(group) = GroupDao::get_group(group_id) else {
            web_connection.send_string("");
            return Ok(());
        };

        if !GroupEditDao::has_session(user_id, group.get_id()) {
            web_connection.send_string("");
            return Ok(());
        }
    } else {
        if !web_connection.session().contains("homeEditSession") {
            web_connection.send_string("");
            return Ok(());
        }
    }

    let widget_id = web_connection.post().get_int("stickieId").unwrap_or(0);
    let mut skin_id = web_connection.post().get_int("skinId").unwrap_or(0);

    let widget = if is_group_edit {
        WidgetDao::get_group_widget(
            widget_id,
            web_connection.session().get_int("groupEditSession"),
        )
    } else {
        WidgetDao::get_home_widget(user_id, widget_id)
    };

    let Some(mut widget) = widget else {
        web_connection.send_string("");
        return Ok(());
    };

    let mut tpl = widget.template(web_connection);
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    if (skin_id == 7 || skin_id == 8) && !player_details.has_club_subscription() {
        skin_id = 1;
    }

    if skin_id == 9
        && player_details.get_rank().map(|rank| rank.rank_id()).unwrap_or(0) < 5
    {
        skin_id = 1;
    }

    widget.set_skin_id(skin_id);
    widget.save();

    web_connection.set_header(
        "X-JSON",
        &format!(
            "{{\"id\":\"{id}\",\"cssClass\":\"n_skin_{css_class}\",\"type\":\"stickie\"}}",
            id = widget.id,
            css_class = widget.get_skin(),
        ),
    );

    tpl.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `stickieDelete(WebConnection)`.
pub fn stickie_delete(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let is_group_edit = web_connection.session().contains("groupEditSession");

    if is_group_edit {
        let group_id = web_connection.session().get_int("groupEditSession");

        let Some(group) = GroupDao::get_group(group_id) else {
            web_connection.send_string("");
            return Ok(());
        };

        if !GroupEditDao::has_session(user_id, group.get_id()) {
            web_connection.send_string("");
            return Ok(());
        }
    } else {
        if !web_connection.session().contains("homeEditSession") {
            web_connection.send_string("");
            return Ok(());
        }
    }

    let stickie_id = web_connection.post().get_int("stickieId").unwrap_or(0);

    if is_group_edit {
        WidgetDao::delete(stickie_id, web_connection.session().get_int("groupEditSession"));
    } else {
        WidgetDao::delete_home_note(stickie_id, user_id);
    }

    web_connection.send_string("SUCCESS");
    Ok(())
}
