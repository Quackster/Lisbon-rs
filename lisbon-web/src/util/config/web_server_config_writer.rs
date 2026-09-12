//! Mirrors `org.alexdev.http.util.config.WebServerConfigWriter`.

use std::collections::HashMap;
use std::io::Write;

use lisbon_server::util::config::writer::config_writer::ConfigWriter;

/// Mirrors `org.alexdev.http.util.config.WebServerConfigWriter`.
pub struct WebServerConfigWriter;

impl WebServerConfigWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ConfigWriter for WebServerConfigWriter {
    /// Mirrors `setConfigurationDefaults()`.
    fn set_configuration_defaults(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("site.directory".to_string(), "tools/www".to_string());

        config.insert("bind.ip".to_string(), "127.0.0.1".to_string());
        config.insert("bind.port".to_string(), "80".to_string());

        config.insert("rcon.ip".to_string(), "127.0.0.1".to_string());
        config.insert("rcon.port".to_string(), "12309".to_string());

        config.insert("mysql.hostname".to_string(), "127.0.0.1".to_string());
        config.insert("mysql.port".to_string(), "3306".to_string());
        config.insert("mysql.username".to_string(), "havana".to_string());
        config.insert("mysql.password".to_string(), "verysecret".to_string());
        config.insert("mysql.database".to_string(), "havana".to_string());

        config.insert("template.directory".to_string(), "tools/www-tpl".to_string());
        config.insert("template.name".to_string(), "default-en".to_string());

        config.insert("page.encoding".to_string(), "utf-8".to_string());
        config
    }

    /// Mirrors `setConfigurationData(Map, PrintWriter)`.
    fn set_configuration_data(
        &self,
        config: &HashMap<String, String>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        writeln!(writer, "[Site]")?;
        writeln!(
            writer,
            "site.directory={}",
            config.get("site.directory").map(String::as_str).unwrap_or("")
        )?;
        writeln!(writer, "")?;
        writeln!(writer, "[Global]")?;
        writeln!(
            writer,
            "bind.ip={}",
            config.get("bind.ip").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "bind.port={}",
            config.get("bind.port").map(String::as_str).unwrap_or("")
        )?;
        writeln!(writer, "")?;
        writeln!(writer, "[Rcon]")?;
        writeln!(
            writer,
            "rcon.ip={}",
            config.get("rcon.ip").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "rcon.port={}",
            config.get("rcon.port").map(String::as_str).unwrap_or("")
        )?;
        writeln!(writer, "")?;
        writeln!(writer, "[Database]")?;
        writeln!(
            writer,
            "mysql.hostname={}",
            config.get("mysql.hostname").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "mysql.port={}",
            config.get("mysql.port").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "mysql.username={}",
            config.get("mysql.username").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "mysql.password={}",
            config.get("mysql.password").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "mysql.database={}",
            config.get("mysql.database").map(String::as_str).unwrap_or("")
        )?;
        writeln!(writer, "")?;
        writeln!(writer, "[Template]")?;
        writeln!(
            writer,
            "template.directory={}",
            config.get("template.directory").map(String::as_str).unwrap_or("")
        )?;
        writeln!(
            writer,
            "template.name={}",
            config.get("template.name").map(String::as_str).unwrap_or("")
        )?;
        writeln!(writer, "")?;
        writeln!(
            writer,
            "page.encoding={}",
            config.get("page.encoding").map(String::as_str).unwrap_or("")
        )?;
        writer.flush()
    }
}
