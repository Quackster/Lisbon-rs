//! Mirrors `org.alexdev.http.util.rcon.RconTask`.

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::server_configuration::ServerConfiguration;

/// Mirrors `org.alexdev.http.util.rcon.RconTask`.
pub struct RconTask {
    header: RconHeader,
    parameters: HashMap<String, String>,
}

impl RconTask {
    /// Mirrors `RconTask(RconHeader, Map<String, Object>)`.
    pub fn new(header: RconHeader, parameters: HashMap<String, String>) -> Self {
        Self { header, parameters }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let host = ServerConfiguration::get_string("rcon.ip");
        let port = ServerConfiguration::get_integer("rcon.port");

        let _ = self.send(&host, port as u16);
    }

    fn send(&self, host: &str, port: u16) -> std::io::Result<()> {
        let mut socket = TcpStream::connect((host, port))?;

        let raw_header = self.header.get_raw_header();
        let mut message: Vec<u8> = Vec::new();

        message.extend_from_slice(&(raw_header.len() as i32).to_be_bytes());
        message.extend_from_slice(raw_header.as_bytes());
        message.extend_from_slice(&(self.parameters.len() as i32).to_be_bytes());

        for (key, value) in &self.parameters {
            message.extend_from_slice(&(key.len() as i32).to_be_bytes());
            message.extend_from_slice(key.as_bytes());

            message.extend_from_slice(&(value.len() as i32).to_be_bytes());
            message.extend_from_slice(value.as_bytes());
        }

        let mut output = Vec::with_capacity(4 + message.len());
        output.extend_from_slice(&(message.len() as i32).to_be_bytes());
        output.extend_from_slice(&message);

        socket.write_all(&output)?;
        socket.flush()?;

        Ok(())
    }
}
