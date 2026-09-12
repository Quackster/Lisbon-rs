//! Mirrors `org.alexdev.http.controllers.site`.

use rand::Rng;

use crate::duckhttpd::TemplateValue;

pub mod account_controller;
pub mod client_controller;
pub mod club_controller;
pub mod collectables_controller;
pub mod community_controller;
pub mod credits_controller;
pub mod faq_controller;
pub mod friend_management_controller;
pub mod games_controller;
pub mod homepage_controller;
pub mod minimail_controller;
pub mod news_controller;
pub mod profile_controller;
pub mod quickmenu_controller;
pub mod recovery_controller;
pub mod register_controller;
pub mod tag_controller;
pub mod site_controller;

/// Mirrors `java.util.UUID.randomUUID().toString()` (the `uuid` crate is not
/// available; a v4-shaped UUID is built from 16 random bytes).
pub fn random_uuid() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];
    for byte in bytes.iter_mut() {
        *byte = rng.gen();
    }
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    let hex: Vec<String> = bytes.iter().map(|byte| format!("{byte:02x}")).collect();

    format!(
        "{}{}{}{}-{}{}-{}{}-{}{}-{}{}{}{}{}{}",
        hex[0], hex[1], hex[2], hex[3], hex[4], hex[5], hex[6], hex[7], hex[8], hex[9],
        hex[10], hex[11], hex[12], hex[13], hex[14], hex[15]
    )
}

/// Placeholder for a list of Java objects whose type does not yet implement
/// `serde::Serialize` (e.g. `Room`), so a null array of the right length is
/// emitted instead of a real payload.
pub fn pending_json(count: usize) -> TemplateValue {
    TemplateValue::json(serde_json::Value::Array(
        vec![serde_json::Value::Null; count],
    ))
}
