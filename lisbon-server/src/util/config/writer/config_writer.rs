//! Mirrors `net.h4bbo.lisbon.util.config.writer.ConfigWriter`.

use std::collections::HashMap;
use std::io::Write;

/// Mirrors the `ConfigWriter` interface.
pub trait ConfigWriter {
    /// Returns the default configuration map (mirrors `setConfigurationDefaults`).
    fn set_configuration_defaults(&self) -> HashMap<String, String>;

    /// Writes configuration data to the provided writer (mirrors
    /// `setConfigurationData(Map, PrintWriter)`).
    fn set_configuration_data(
        &self,
        config: &HashMap<String, String>,
        writer: &mut dyn Write,
    ) -> std::io::Result<()>;
}
