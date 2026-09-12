//! Mirrors `org.alexdev.http.util.RconUtil`.

use std::collections::HashMap;

use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::havana_web::HavanaWeb;
use crate::util::rcon::rcon_task::RconTask;

/// Mirrors `org.alexdev.http.util.RconUtil`.
pub struct RconUtil;

impl RconUtil {
    /// Mirrors `sendCommand(RconHeader, Map<String, Object>)`.
    pub fn send_command(header: RconHeader, parameters: HashMap<String, String>) {
        let rcon_task = RconTask::new(header, parameters);
        HavanaWeb::get_executor().execute(move || rcon_task.run());
    }
}
