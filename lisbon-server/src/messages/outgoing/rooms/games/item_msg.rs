//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.games.ITEMMSG`.

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct ITEMMSG {
    commands: Vec<String>,
    delimitate: bool,
}

impl ITEMMSG {
    /// Mirrors the `ITEMMSG(String[])` constructor.
    pub fn new_commands(commands: &[&str]) -> Self {
        Self {
            commands: commands.iter().map(|command| command.to_string()).collect(),
            delimitate: true,
        }
    }

    /// Mirrors the `ITEMMSG(String)` constructor.
    pub fn new_command(command: &str) -> Self {
        Self {
            commands: vec![command.to_string()],
            delimitate: false,
        }
    }
}

impl MessageComposer for ITEMMSG {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for value in &self.commands {
            if self.delimitate {
                response.write_delimeter(value, 13u8);
            } else {
                response.write(value);
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        144
    }
}
