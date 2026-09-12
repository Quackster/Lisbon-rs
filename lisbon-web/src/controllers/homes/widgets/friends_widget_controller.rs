//! Mirrors `org.alexdev.http.controllers.homes.widgets.FriendsWidgetController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;

use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `friendsearchpaging(WebConnection)`.
pub fn friendsearchpaging(
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

    let pages = if search_string.trim().is_empty() {
        widget.get_friends_pages()
    } else {
        widget.get_friends_pages_search(&search_string)
    };

    let friends_list = widget.get_friends_list(&search_string, page_number);

    let mut template = web_connection.template("homes/widget/habblet/friendsearchpaging");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.set("pages", TemplateValue::of(pages));
    template.set("friends", TemplateValue::of(widget.get_friends_amount()));
    template.set("friendsList", TemplateValue::of(friends_list));
    template.set("currentPage", TemplateValue::of(page_number));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `avatarinfo(WebConnection)`.
pub fn avatarinfo(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.post().get_int("anAccountId").unwrap_or(0);

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let mut template = web_connection.template("homes/widget/habblet/avatarinfo");
    template.set("avatar", TemplateValue::of(player_details));
    template.render_html().ok();
    Ok(())
}
