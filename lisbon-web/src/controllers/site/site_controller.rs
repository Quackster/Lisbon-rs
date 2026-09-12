//! Mirrors `org.alexdev.http.controllers.site.SiteController`.

use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `pixels(WebConnection)`.
pub fn pixels(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().contains("authenticated") {
        return Ok(());
    }

    let mut template = web_connection.template("pixels");
    web_connection.session().set("page", SessionValue::Str("credits".to_string()));
    template.render();

    Ok(())
}
