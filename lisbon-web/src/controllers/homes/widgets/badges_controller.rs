//! Mirrors `org.alexdev.http.controllers.homes.widgets.BadgesController`.

use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `badgepaging(WebConnection)`.
pub fn badge_paging(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);
    let mut page_number = web_connection.post().get_int("pageNumber").unwrap_or(0);

    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let pages = widget.get_badge_list();
    let mut show_last = true;

    if page_number > pages.len() as i32 {
        page_number = pages.len() as i32;
    }

    if page_number >= pages.len() as i32 {
        show_last = false;
    }

    if page_number <= 0 {
        page_number = 1;
    }

    // Mirrors the Java fallback to the first page when the requested page
    // is absent.
    let badge_list = pages
        .get(&((page_number - 1) as usize))
        .cloned()
        .or_else(|| pages.get(&0).cloned())
        .unwrap_or_default();

    let mut template = web_connection.template("homes/widget/habblet/badgepaging");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.set("pages", TemplateValue::of(pages.len() as i32));
    template.set("showLast", TemplateValue::of(show_last));
    template.set("badgeList", TemplateValue::of(badge_list));
    template.set("currentPage", TemplateValue::of(page_number));
    template.render_html().ok();
    Ok(())
}
