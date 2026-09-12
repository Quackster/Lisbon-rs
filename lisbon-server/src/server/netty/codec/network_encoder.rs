//! Mirrors `net.h4bbo.lisbon.server.netty.codec.NetworkEncoder`.
//!
//! Java extends `MessageToMessageEncoder<Object>` and encodes either a
//! `MessageComposer` or a raw `String` into a `ByteBuf`. Here the outbound
//! object is a `&dyn MessageComposer` (the raw-`String` form is offered via
//! `encode_string`).

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::netty_response::NettyResponse;
use crate::util::config::server_configuration::ServerConfiguration;

/// Encodes outbound game frames.
pub struct NetworkEncoder;

impl NetworkEncoder {
    /// Mirrors the `MessageComposer` branch of `encode(...)`.
    ///
    /// Composes the message into a `NettyResponse`, appends the terminator
    /// byte when the response was not finalised, and returns the serialised
    /// frame. Returns `None` if there is nothing to send.
    ///
    /// Port note: the Java `try/catch` around `msg.compose(response)` (which
    /// logs the composing user) is inapplicable — `MessageComposer::compose`
    /// does not fail in the Rust port.
    pub fn encode_composer(msg: &dyn MessageComposer) -> Option<Vec<u8>> {
        let mut response = NettyResponse::new(msg.get_header());

        msg.compose(&mut response);

        let mut buffer = response.take_buffer();
        if !response.is_finalised() {
            buffer.push(1);
            response.set_finalised(true);
        }

        if ServerConfiguration::get_boolean("log.sent.packets") {
            tracing::info!("SENT: {} / {}", msg.get_header(), response.get_body_string());
        }

        Some(buffer)
    }

    /// Mirrors the `String` branch of `encode(...)`.
    pub fn encode_string(value: &str) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
}
