//! Mirrors `net.h4bbo.lisbon.server.netty.connections.ConnectionVersionRule`.

/// A (port → client protocol version) rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectionVersionRule {
    port: i32,
    version: i32,
}

impl ConnectionVersionRule {
    /// Mirrors the `ConnectionVersionRule(int, int)` constructor.
    pub fn new(port: i32, version: i32) -> Self {
        Self { port, version }
    }

    /// Mirrors `getPort()`.
    pub fn get_port(&self) -> i32 {
        self.port
    }

    /// Mirrors `getVersion()`.
    pub fn get_version(&self) -> i32 {
        self.version
    }
}
