//! Mirrors `org.alexdev.http.controllers.site.FaqController`.

use crate::duckhttpd::WebConnection;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `faq(WebConnection)`.
pub fn faq(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut template = web_connection.template("faq");
    template.render();

    Ok(())
}
