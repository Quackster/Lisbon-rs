//! Mirrors `net.h4bbo.lisbon.server.rcon.messages.RconMessage`.

use std::collections::HashMap;

use super::rcon_header::RconHeader;

pub struct RconMessage {
    header: Option<RconHeader>,
    values: HashMap<String, String>,
}

impl RconMessage {
    /// Mirrors the `RconMessage(String, Map)` constructor.
    pub fn new(header: &str, values: HashMap<String, String>) -> Self {
        Self {
            header: RconHeader::get_by_header(header),
            values,
        }
    }

    /// Mirrors `getHeader()`.
    pub fn get_header(&self) -> Option<RconHeader> {
        self.header
    }

    /// Mirrors `getValues()`.
    pub fn get_values(&self) -> &HashMap<String, String> {
        &self.values
    }
}
