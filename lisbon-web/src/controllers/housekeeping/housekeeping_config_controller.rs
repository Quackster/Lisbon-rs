//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingConfigController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::settings_dao::SettingsDao;
use lisbon_server::game::player::player_rank::PlayerRank;

use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::util::config_entry::ConfigEntry;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

/// Mirrors `configurations(WebConnection)`.
pub fn configurations(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/configurations");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        "configuration",
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    if web_connection.post().queries().len() > 0 {
        let entries: Vec<(String, String)> =
            web_connection.post().get_values().into_iter().collect();
        SettingsDao::update_settings(&entries);

        // Reload config
        // The Java `GameConfiguration` reload via `WebSettingsConfigWriter`
        // is commented out in the source.

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "All configuration values have been saved successfully! It will take effect within 30 seconds."
                    .into(),
            ),
        );
    }

    let mut settings: Vec<ConfigEntry> = SettingsDao::get_all_settings()
        .into_iter()
        .map(|(key, value)| ConfigEntry::new(key, value))
        .collect();

    settings.sort_by(|a, b| a.get_key().cmp(b.get_key()));

    tpl.set("pageName", TemplateValue::of("Configurations"));
    tpl.set(
        "configs",
        TemplateValue::json(
serde_json::Value::Array(            settings
                .iter()
                .map(|setting| serde_json::json!([setting.get_key(), setting.get_value()]))
                .collect::<Vec<_>>(),

        )        ),
    );
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
