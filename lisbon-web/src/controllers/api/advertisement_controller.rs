//! Mirrors `org.alexdev.http.controllers.api.AdvertisementController`.

use lisbon_server::game::ads::ad_manager::AdManager;

use crate::duckhttpd::WebConnection;

/// Mirrors `getImg(WebConnection)`.
pub fn get_img(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.get().contains("ad") {
        web_connection.send_string("");
        return Ok(());
    }

    let ad_id = match web_connection.get().get_int("ad") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    match AdManager::get_instance().get_ad(ad_id) {
        Some(advertisement) => web_connection.redirect(advertisement.get_image()),
        None => web_connection.send_string(""),
    }

    Ok(())
}

/// Mirrors `getUrl(WebConnection)`.
pub fn get_url(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.get().contains("ad") {
        web_connection.send_string("");
        return Ok(());
    }

    let ad_id = match web_connection.get().get_int("ad") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    match AdManager::get_instance().get_ad(ad_id) {
        Some(advertisement) => web_connection.redirect(advertisement.get_url()),
        None => web_connection.send_string(""),
    }

    Ok(())
}
