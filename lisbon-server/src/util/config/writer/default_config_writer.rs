//! Mirrors `net.h4bbo.lisbon.util.config.writer.DefaultConfigWriter`.

use std::collections::HashMap;
use std::io::Write;

use super::ConfigWriter;

fn get(config: &HashMap<String, String>, key: &str) -> String {
    config.get(key).cloned().unwrap_or_default()
}

pub struct DefaultConfigWriter;

impl DefaultConfigWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DefaultConfigWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigWriter for DefaultConfigWriter {
    fn set_configuration_defaults(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("server.bind".into(), "127.0.0.1".into());
        config.insert("server.port".into(), "12321".into());
        config.insert("mus.bind".into(), "127.0.0.1".into());
        config.insert("mus.port".into(), "12322".into());
        config.insert("rcon.bind".into(), "127.0.0.1".into());
        config.insert("rcon.port".into(), "12309".into());
        config.insert("log.connections".into(), "true".into());
        config.insert("log.sent.packets".into(), "false".into());
        config.insert("log.received.packets".into(), "false".into());
        config.insert("mysql.hostname".into(), "127.0.0.1".into());
        config.insert("mysql.port".into(), "3306".into());
        config.insert("mysql.username".into(), "lisbon".into());
        config.insert("mysql.password".into(), "verysecret".into());
        config.insert("mysql.database".into(), "lisbon".into());
        config.insert("debug".into(), "false".into());
        config
    }

    fn set_configuration_data(
        &self,
        config: &HashMap<String, String>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        writeln!(writer, "[Server]")?;
        writeln!(writer, "server.bind={}", get(config, "server.bind"))?;
        writeln!(writer, "server.port={}", get(config, "server.port"))?;
        writeln!(writer, "")?;
        writeln!(writer, "[Rcon]")?;
        writeln!(writer, "rcon.bind={}", get(config, "rcon.bind"))?;
        writeln!(writer, "rcon.port={}", get(config, "rcon.port"))?;
        writeln!(writer, "")?;
        writeln!(writer, "[Mus]")?;
        writeln!(writer, "mus.port={}", get(config, "mus.port"))?;
        writeln!(writer, "")?;
        writeln!(writer, "[Database]")?;
        writeln!(writer, "mysql.hostname={}", get(config, "mysql.hostname"))?;
        writeln!(writer, "mysql.port={}", get(config, "mysql.port"))?;
        writeln!(writer, "mysql.username={}", get(config, "mysql.username"))?;
        writeln!(writer, "mysql.password={}", get(config, "mysql.password"))?;
        writeln!(writer, "mysql.database={}", get(config, "mysql.database"))?;
        writeln!(writer, "")?;
        writeln!(writer, "[Logging]")?;
        writeln!(writer, "log.received.packets={}", get(config, "log.received.packets"))?;
        writeln!(writer, "log.sent.packets={}", get(config, "log.sent.packets"))?;
        writeln!(writer, "")?;
        writeln!(writer, "[Console]")?;
        write!(writer, "debug={}", get(config, "debug"))?;
        writer.flush()
    }
}
