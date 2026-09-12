//! Mirrors `org.alexdev.http.controllers.site.CreditsController`.

use chrono::{Datelike, Local, Months};

use lisbon_server::dao::mysql::transaction_dao::TransactionDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `credits(WebConnection)`.
pub fn credits(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::create_key(web_connection, "/credits");

    let mut template = web_connection.template("credits");
    web_connection.session().set("page", SessionValue::Str("credits".to_string()));
    template.render();

    Ok(())
}

/// Mirrors `transactions(WebConnection)`.
pub fn transactions(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let present = chrono::Local::now();
    let get = web_connection.get();

    let mut template = web_connection.template("credits_history");

    let details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            template.render();
            return Ok(());
        }
    };

    let mut user_id = details.get_id();
    let view_all = details
        .get_rank()
        .map_or(false, |rank| rank.rank_id() >= PlayerRank::Moderator.rank_id());

    if view_all {
        if let Some(value) = get.get_int("userId") {
            user_id = value;
        }
    }

    let has_date_parameter = get.contains("period")
        && get
            .get_string("period")
            .map_or(false, |value| DateUtil::get_from_format("yyyy-MM-dd", &value) > 0);

    let mut current = present;

    if has_date_parameter {
        let time = DateUtil::get_from_format(
            "yyyy-MM-dd",
            get.get_string("period").as_deref().unwrap_or_default(),
        );
        if let Some(from_timestamp) = chrono::DateTime::from_timestamp(time, 0) {
            current = from_timestamp.with_timezone(&Local);
        }
    }

    let previous = current - Months::new(1);
    let future = current + Months::new(1);

    let year = current.year();
    let month = current.month() as i32;
    let transactions_this_month = TransactionDao::get_transactions(user_id, month, year, view_all);

    template.set(
        "canGoNext",
        TemplateValue::of(current.month() != present.month() || current.year() != present.year()),
    );

    template.set("previousYear", TemplateValue::of(previous.year()));
    template.set(
        "previousMonth",
        TemplateValue::of(previous.format("%B").to_string()),
    );
    template.set(
        "previousNumericalMonth",
        TemplateValue::of(previous.month() as i32),
    );

    template.set("futureYear", TemplateValue::of(future.year()));
    template.set("futureMonth", TemplateValue::of(future.format("%B").to_string()));
    template.set("futureNumericalMonth", TemplateValue::of(future.month() as i32));

    web_connection.session().set("page", SessionValue::Str("credits".to_string()));

    template.set("currentYear", TemplateValue::of(year));
    template.set(
        "currentMonth",
        TemplateValue::of(current.format("%B").to_string()),
    );

    template.set("transactions", TemplateValue::of(&transactions_this_month));
    template.render();

    Ok(())
}
