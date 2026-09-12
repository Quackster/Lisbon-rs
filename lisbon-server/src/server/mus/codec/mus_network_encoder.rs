//! Mirrors `net.h4bbo.lisbon.server.mus.codec.MusNetworkEncoder`.

use crate::server::mus::mus_util::MusUtil;
use crate::server::mus::streams::mus_message::MusMessage;
use crate::server::mus::streams::mus_types as MusTypes;
use crate::util::date_util::DateUtil;

pub struct MusNetworkEncoder;

impl MusNetworkEncoder {
    /// Mirrors `encode(ChannelHandlerContext, MusMessage, List<Object>)`.
    ///
    /// Stamps the system sender / receivers / timestamp (as the Java encoder
    /// does) and serialises the frame as
    /// `b'r', 0, <4-byte body length>, <body>`.
    pub fn encode(msg: &mut MusMessage) -> Vec<u8> {
        msg.set_sender_id("System");
        msg.set_receivers(vec!["*".to_string()]);
        msg.set_timestamp(DateUtil::get_current_time_seconds() as i64);

        let mut temporary_buffer: Vec<u8> = Vec::new();
        temporary_buffer.extend_from_slice(&msg.get_error_code().to_be_bytes());
        temporary_buffer.extend_from_slice(&(msg.get_timestamp() as i32).to_be_bytes());

        MusUtil::write_even_padded_string(&mut temporary_buffer, msg.get_subject());
        MusUtil::write_even_padded_string(&mut temporary_buffer, msg.get_sender_id());

        let receivers = msg.get_receivers().to_vec();
        temporary_buffer.extend_from_slice(&(receivers.len() as i32).to_be_bytes());

        for receiver in &receivers {
            MusUtil::write_even_padded_string(&mut temporary_buffer, receiver);
        }

        temporary_buffer.extend_from_slice(&msg.get_content_type().to_be_bytes());

        // Content
        if msg.get_content_type() != MusTypes::VOID {
            if msg.get_content_type() == MusTypes::INTEGER {
                temporary_buffer.extend_from_slice(&msg.get_content_int().to_be_bytes());
            } else if msg.get_content_type() == MusTypes::STRING {
                MusUtil::write_even_padded_string(
                    &mut temporary_buffer,
                    msg.get_content_string(),
                );
            } else if msg.get_content_type() == MusTypes::PROPLIST {
                if let Some(prop_list) = msg.get_content_prop_list() {
                    MusUtil::write_prop_list(&mut temporary_buffer, prop_list);
                }
            } else {
                tracing::warn!(
                    "Unsupported MusMessage content type {}!",
                    msg.get_content_type()
                );
            }
        }

        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(b'r');
        buffer.push(0);
        buffer.extend_from_slice(&(temporary_buffer.len() as i32).to_be_bytes());
        buffer.extend_from_slice(&temporary_buffer);

        buffer
    }
}
