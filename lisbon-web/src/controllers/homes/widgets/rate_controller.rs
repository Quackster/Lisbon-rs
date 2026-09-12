//! Mirrors `org.alexdev.http.controllers.homes.widgets.RateController`.

use crate::dao::rating_dao::RatingDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{TemplateValue, WebConnection};

/// Mirrors `rate(WebConnection)`.
pub fn rate(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let mut widget_id = -1;
    let mut rating = -1;

    if let Some(value) = web_connection.get().get_int("ratingId") {
        widget_id = value;
    }

    if let Some(value) = web_connection.get().get_int("givenRate") {
        rating = value;
    }

    if rating < 1 || rating > 5 {
        web_connection.send_string("");
        return Ok(());
    }

    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let home_id = widget.user_id;

    if home_id == user_id {
        web_connection.send_string("");
        return Ok(());
    }

    if RatingDao::has_rated(user_id, home_id) {
        web_connection.send_string("");
        return Ok(());
    }

    RatingDao::rate(user_id, home_id, rating);

    let mut template = web_connection.template("homes/widget/habblet/rate");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `resetRating(WebConnection)`.
pub fn reset_rating(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let widget_id = web_connection.get().get_int("ratingId").unwrap_or(-1);

    let Some(widget) = WidgetDao::get_widget(widget_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let home_id = widget.user_id;

    if home_id != user_id {
        web_connection.send_string("");
        return Ok(());
    }

    RatingDao::delete_rating(home_id);

    let mut template = web_connection.template("homes/widget/habblet/rate");
    template.set("sticker", TemplateValue::json(serde_json::to_value(&widget).unwrap_or_default()));
    template.render_html().ok();
    Ok(())
}
