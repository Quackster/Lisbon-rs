//! Mirrors `org.alexdev.http.controllers.homes.widgets.MemberWidgetController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;

use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `membersearchpaging(WebConnection)`.
pub fn membersearchpaging(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let widget_id = web_connection.post().get_int("widgetId").unwrap_or(0);
    let mut page_number = 1;

    if let Some(value) = web_connection.post().get_int("pageNumber") {
        page_number = value;
    }

    if page_number <= 0 {
        page_number = 1;
    }

    let search_string = web_connection.post().get_string("searchString").unwrap_or_default();

    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let pages = widget.get_members_pages();
    let member_list = widget.get_members_list(&search_string, page_number);

    let mut template = web_connection.template("homes/widget/habblet/membersearchpaging");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.set("pages", TemplateValue::of(pages));
    template.set("members", TemplateValue::of(widget.get_members_amount()));
    template.set("membersList", TemplateValue::of(member_list));
    template.set("currentPage", TemplateValue::of(page_number));

    if let Some(group) = GroupDao::get_group(widget.group_id) {
        template.set("group", TemplateValue::of(group));
    }
    template.render_html().ok();
    Ok(())
}
