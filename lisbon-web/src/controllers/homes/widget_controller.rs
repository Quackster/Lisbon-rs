//! Mirrors `org.alexdev.http.controllers.homes.WidgetController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;

use crate::dao::group_edit_dao::GroupEditDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `editWidget(WebConnection)`.
pub fn edit_widget(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
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

    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);
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

    if widget
        .get_product()
        .map(|product| product.is_group_widget() || product.is_home_widget())
        .unwrap_or(false)
    {
        web_connection.set_header(
            "X-JSON",
            &format!(
                "{{\"id\":\"{id}\",\"cssClass\":\"w_skin_{css_class}\",\"type\":\"widget\"}}",
                id = widget.id,
                css_class = widget.get_skin(),
            ),
        );
    }

    tpl.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `placeSticker(WebConnection)`.
pub fn place_sticker(
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

    let widget_id = web_connection.post().get_int("selectedStickerId").unwrap_or(0);
    let mut zindex = web_connection.post().get_int("zindex").unwrap_or(0);

    if zindex < 0 || zindex > 100 {
        zindex = 0;
    }

    let mut widget = match WidgetDao::get_inventory_widget(user_id, widget_id) {
        Some(widget) => widget,
        None => {
            // Java NPEs on a null widget.
            return Ok(());
        }
    };

    widget.set_x(20);
    widget.set_y(30);
    widget.set_z(zindex);

    if is_group_edit {
        widget.set_group_id(web_connection.session().get_int("groupEditSession"));
    }

    widget.set_placed(true);
    widget.save();

    web_connection.set_header("X-JSON", &format!("[\"{}\"]", widget.id));

    let mut tpl = widget.template(web_connection);
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `placeWidget(WebConnection)`.
pub fn place_widget(
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

    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);
    let mut zindex = web_connection.post().get_int("zindex").unwrap_or(0);

    if zindex < 0 || zindex > 100 {
        zindex = 0;
    }

    let widget = if is_group_edit {
        WidgetDao::get_group_widget(
            widget_id,
            web_connection.session().get_int("groupEditSession"),
        )
    } else {
        WidgetDao::get_home_widget(user_id, widget_id)
    };

    let Some(mut widget) = widget else {
        // Java NPEs on a null widget.
        return Ok(());
    };

    widget.set_x(10);
    widget.set_y(10);
    widget.set_z(zindex);

    if is_group_edit {
        widget.set_group_id(web_connection.session().get_int("groupEditSession"));
    }

    widget.set_placed(true);
    widget.save();

    web_connection.set_header("X-JSON", &format!("[\"{}\"]", widget.id));

    let mut tpl = widget.template(web_connection);

    if is_group_edit {
        if let Some(group) = GroupDao::get_group(widget.group_id) {
            tpl.set("group", TemplateValue::of(group));
        }
    }

    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `removeSticker(WebConnection)`.
pub fn remove_sticker(
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

    let widget_id = web_connection.post().get_int("stickerId").unwrap_or(0);

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

    widget.set_x(0);
    widget.set_y(0);
    widget.set_z(0);
    widget.set_group_id(0);
    widget.set_placed(false);
    widget.save();

    web_connection.send_string("SUCCESS");
    Ok(())
}

/// Mirrors `removeWidget(WebConnection)`.
pub fn remove_widget(
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

    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);

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

    if widget
        .get_product()
        .map(|product| {
            product.data.eq_ignore_ascii_case("groupinfowidget")
                || product.data.eq_ignore_ascii_case("profilewidget")
        })
        .unwrap_or(false)
    {
        web_connection.send_string("");
        return Ok(());
    }

    widget.set_x(0);
    widget.set_y(0);
    widget.set_z(0);

    if is_group_edit {
        widget.set_group_id(web_connection.session().get_int("groupEditSession"));
    }

    widget.set_placed(false);
    widget.save();

    web_connection.send_string("SUCCESS");
    Ok(())
}
