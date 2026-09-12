//! Mirrors `org.alexdev.http.controllers.site.HomepageController`.

use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::server::watchdog::TAG_CLOUD_20;
use crate::util::home_util::HomeUtil;
use crate::util::html_util::HtmlUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `homepage(WebConnection)`.
pub fn homepage(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/me");
        return Ok(());
    }

    let remember_me = web_connection.get().get_string("rememberme").as_deref() == Some("true");
    let username = match web_connection.get().get_string("username") {
        Some(value) => HtmlUtil::remove_html_tags(&value),
        None => String::new(),
    };

    let mut template = web_connection
        .template(&GameConfiguration::get_instance().get_string("homepage.template.file"));
    template.set("rememberMe", TemplateValue::of(remember_me));
    template.set("username", TemplateValue::of(&username));
    template.set("tagCloud", TemplateValue::of(TAG_CLOUD_20.read().clone()));

    let is_valentines_month = matches!(DateUtil::get_current_date("M").parse::<i32>(), Ok(2))
        && DateUtil::get_current_date("DD")
            .parse::<i32>()
            .map_or(false, |value| value <= 16);
    template.set("isValentinesMonth", TemplateValue::of(is_valentines_month));
    template.set(
        "randomValentinesImage",
        TemplateValue::of(HomeUtil::get_random_valentines_image()),
    );

    template.render();

    web_connection.session().delete("alertMessage");

    Ok(())
}

/// Mirrors `maintenance(WebConnection)`.
pub fn maintenance(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("maintenance");
    template.render();

    Ok(())
}
