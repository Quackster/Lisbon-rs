//! Mirrors `org.alexdev.http.util.config.WebSettingsConfigWriter`.

use std::collections::HashMap;
use std::io::Write;

use lisbon_server::util::config::writer::config_writer::ConfigWriter;
use lisbon_server::util::config::writer::game_config_writer::GameConfigWriter;

/// Mirrors `org.alexdev.http.util.config.WebSettingsConfigWriter`.
pub struct WebSettingsConfigWriter;

impl WebSettingsConfigWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ConfigWriter for WebSettingsConfigWriter {
    /// Mirrors `setConfigurationDefaults()`.
    fn set_configuration_defaults(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("site.name".to_string(), "Habbo".to_string());
        config.insert("site.path".to_string(), "http://localhost".to_string());
        config.insert(
            "static.content.path".to_string(),
            "http://localhost".to_string(),
        );

        config.insert("hotel.check.online".to_string(), "true".to_string());

        config.insert("loader.game.ip".to_string(), "127.0.0.1".to_string());
        config.insert("loader.game.port".to_string(), "12321".to_string());

        config.insert("loader.mus.ip".to_string(), "127.0.0.1".to_string());
        config.insert("loader.mus.port".to_string(), "12322".to_string());

        config.insert(
            "loader.dcr".to_string(),
            "http://localhost/dcr/r26_20080915_0408_7984_61ccb5f8b8797a3aba62c1fa2ca80169/habbo.dcr".to_string(),
        );
        config.insert(
            "loader.external.variables".to_string(),
            "http://localhost/gamedata/external_variables.txt?".to_string(),
        );
        config.insert(
            "loader.external.texts".to_string(),
            "http://localhost/gamedata/external_texts.txt?".to_string(),
        );

        config.insert("registration.disabled".to_string(), "false".to_string());
        config.insert("collectables.page".to_string(), "51".to_string());

        config.insert("group.purchase.cost".to_string(), "20".to_string());
        config.insert(
            "group.default.badge".to_string(),
            "b0503Xs09114s05013s05015".to_string(),
        );

        config.insert("hot.groups.community.limit".to_string(), "8".to_string());
        config.insert("hot.groups.limit".to_string(), "10".to_string());

        config.insert("discussions.per.page".to_string(), "10".to_string());
        config.insert("discussions.replies.per.page".to_string(), "10".to_string());

        config.insert(
            "alerts.gift.message".to_string(),
            "A new gift has arrived. This time you received a %item_name%.".to_string(),
        );
        config.insert(
            "homepage.template.file".to_string(),
            "index".to_string(),
        );

        config.insert("free.month.hc.registration".to_string(), "true".to_string());

        config.insert("max.tags.users".to_string(), "8".to_string());
        config.insert("max.tags.groups".to_string(), "20".to_string());

        config.insert("trade.email.verification".to_string(), "false".to_string());
        config.insert("email.smtp.enable".to_string(), "false".to_string());
        config.insert(
            "email.static.content.path".to_string(),
            "http://localhost".to_string(),
        );

        config.insert("email.smtp.host".to_string(), String::new());
        config.insert("email.smtp.port".to_string(), "465".to_string());

        config.insert("email.smtp.login.username".to_string(), String::new());
        config.insert("email.smtp.login.password".to_string(), String::new());

        config.insert("email.smtp.from.email".to_string(), String::new());
        config.insert("email.smtp.from.name".to_string(), String::new());

        config.insert("maintenance".to_string(), "false".to_string());

        config.extend(GameConfigWriter::new().set_configuration_defaults());
        config
    }

    /// Mirrors `setConfigurationData(Map, PrintWriter)`.
    fn set_configuration_data(
        &self,
        _config: &HashMap<String, String>,
        _writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        Ok(())
    }
}
