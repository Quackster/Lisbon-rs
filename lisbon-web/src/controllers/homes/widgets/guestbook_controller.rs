//! Mirrors `org.alexdev.http.controllers.homes.widgets.GuestbookController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::guestbook_dao::GuestbookDao;
use crate::dao::widget_dao::WidgetDao;
use crate::game::homes::guestbook_entry::GuestbookEntry;
use crate::game::stickers::sticker_type::StickerType;
use rand::Rng;
use crate::duckhttpd::{ResponseBuilder, TemplateValue, WebConnection};
use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

/// Mirrors `preview(WebConnection)`.
pub fn preview(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut message = BBCode::format(
        &HtmlUtil::escape(&BBCode::normalise(
            &web_connection.post().get_string("message").unwrap_or_default(),
        )),
        false,
    );

    if message.chars().count() > 200 {
        message = message.chars().take(200).collect();
    }

    let mut template = web_connection.template("homes/widget/guestbook/preview");
    template.set("message", TemplateValue::of(message));
    if let Some(player_details) = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        template.set("author", TemplateValue::of(player_details));
    }
    template.set(
        "formattedDate",
        TemplateValue::of(DateUtil::get_friendly_date(
            DateUtil::get_current_time_seconds() as i64,
        )),
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `add(WebConnection)`.
pub fn add(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);
    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !widget
        .get_product()
        .map(|product| product.data.to_lowercase() == "guestbookwidget")
        .unwrap_or(false)
    {
        web_connection.send_string("");
        return Ok(());
    }

    let mut message = web_connection.post().get_string("message").unwrap_or_default();

    if message.chars().count() > 200 {
        message = message.chars().take(200).collect();
    }

    if !widget.is_placed {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    if !widget.is_posting_allowed(user_id) {
        web_connection.send_string("");
        return Ok(());
    }

    let mut home_id = 0;
    let mut group_id = 0;

    let product_type = widget.get_product().and_then(|product| product.get_type());

    if product_type == Some(StickerType::GroupWidget) {
        group_id = widget.group_id;
    } else if product_type == Some(StickerType::HomeWidget) {
        home_id = widget.user_id;

        if home_id != user_id {
            PlayerStatisticsDao::increment_statistic(
                home_id,
                PlayerStatistic::GuestbookUnreadMessages,
                1,
            );
        }
    }

    let guestbook_entry = if WordfilterManager::filter_sentence(&message) == message {
        GuestbookDao::create(user_id, home_id, group_id, &message)
    } else {
        GuestbookEntry::new(
            rand::thread_rng().gen_range(0..i32::MAX),
            user_id,
            home_id,
            group_id,
            &message,
            DateUtil::get_current_time_seconds() as i64,
        )
    };

    let mut template = web_connection.template("homes/widget/guestbook/add");
    template.set("entry", TemplateValue::of(guestbook_entry.clone()));
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.set(
        "canDeleteEntries",
        TemplateValue::of(widget.can_delete_entries(user_id)),
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `remove(WebConnection)`.
pub fn remove(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut entry_id = -1;
    let mut widget_id = -1;

    if let Some(value) = web_connection.post().get_int("entryId") {
        entry_id = value;
    }

    if let Some(value) = web_connection.post().get_int("widgetId") {
        widget_id = value;
    }

    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !widget
        .get_product()
        .map(|product| product.data.to_lowercase() == "guestbookwidget")
        .unwrap_or(false)
        || !widget.is_placed
    {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let Some(entry) = GuestbookDao::get_entry(entry_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !widget.can_delete_entries(user_id) && entry.user_id != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    let mut home_id = 0;
    let mut group_id = 0;

    let product_type = widget.get_product().and_then(|product| product.get_type());

    if product_type == Some(StickerType::GroupWidget) {
        group_id = widget.group_id;
    } else if product_type == Some(StickerType::HomeWidget) {
        home_id = widget.user_id;
    }

    GuestbookDao::remove(entry_id, home_id, group_id);

    let mut template = web_connection.template("homes/widget/guestbook_widget");
    template.set(
        "editMode",
        TemplateValue::of(
            web_connection.session().contains("homeEditSession")
                || web_connection.session().contains("groupEditSession"),
        ),
    );
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `configure(WebConnection)`.
pub fn configure(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);

    let Some(mut widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !widget
        .get_product()
        .map(|product| product.data.to_lowercase() == "guestbookwidget")
        .unwrap_or(false)
        || !widget.is_placed
    {
        web_connection.send_string("");
        return Ok(());
    }

    let mut owner_id = 0;

    let product_type = widget.get_product().and_then(|product| product.get_type());

    if product_type == Some(StickerType::GroupWidget) {
        owner_id = GroupDao::get_group_owner(widget.group_id);
    } else if product_type == Some(StickerType::HomeWidget) {
        owner_id = widget.user_id;
    }

    if owner_id != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    if widget.get_guestbook_state().eq_ignore_ascii_case("private") {
        widget.set_extra_data(Some("public".to_string()));
    } else {
        widget.set_extra_data(Some("private".to_string()));
    }

    widget.save();

    web_connection.send(
        ResponseBuilder::create_with_content_type(
            "text/javascript",
            concat!(
                "var el = $(\"guestbook-type\");\n",
                "if (el) {\n",
                "\tif (el.hasClassName(\"public\")) {\n",
                "\t\tel.className = \"private\";\n",
                "\t\tnew Effect.Pulsate(el,\n",
                "\t\t\t{ duration: 1.0, afterFinish : function() { Element.setOpacity(el, 1); } }\n",
                "\t\t);\t\t\t\t\t\t\n",
                "\t} else {\t\t\t\t\t\t\n",
                "\t\tnew Effect.Pulsate(el,\n",
                "\t\t\t{ duration: 1.0, afterFinish : function() { Element.setOpacity(el, 0); el.className = \"public\"; } }\n",
                "\t\t);\t\t\t\t\t\t\n",
                "\t}\n",
                "}",
            ),
        ),
    );

    Ok(())
}
