//! Mirrors `net.h4bbo.lisbon.server.mus.codec.MusNetworkDecoder`.
//!
//! Java extends `ByteArrayDecoder` and is preceded in the pipeline by a
//! `LengthFieldBasedFrameDecoder(1683226630, 2, 4, 0, 0)`, i.e. each frame is
//! `2` header bytes + `4`-byte (big-endian) length + `length` body bytes. Here
//! the framing is folded into `decode`, which pulls whole frames out of the
//! accumulated `Vec<u8>`.

use crate::server::mus::mus_util::{MusUtil, Reader};
use crate::server::mus::streams::mus_message::MusMessage;
use crate::server::mus::streams::mus_types as MusTypes;

pub struct MusNetworkDecoder;

impl MusNetworkDecoder {
    /// Mirrors `decode(ChannelHandlerContext, ByteBuf, List<Object>)`.
    ///
    /// Returns `Some((message, should_close))` when a full frame was decoded
    /// (`should_close` is `true` when the header tag is not `'r'`, mirroring
    /// `ctx.channel().close()`), or `None` when more bytes are required.
    pub fn decode(buf: &mut Vec<u8>) -> Option<(MusMessage, bool)> {
        if buf.len() < 6 {
            return None;
        }

        let header_tag = buf[0];
        let size = i32::from_be_bytes([buf[2], buf[3], buf[4], buf[5]]);
        if size < 0 {
            return None;
        }

        let total = 6 + size as usize;
        if buf.len() < total {
            return None;
        }

        if header_tag != b'r' {
            buf.drain(..total);
            return Some((MusMessage::new(), true));
        }

        let body = buf[6..total].to_vec();
        buf.drain(..total);

        let mut reader = Reader::new(&body);
        let mut message = MusMessage::new();
        message.set_error_code(reader.read_int());
        message.set_timestamp(reader.read_int() as i64);
        let subject = MusUtil::read_even_padded_string(&mut reader);
        message.set_subject(&subject);
        let sender_id = MusUtil::read_even_padded_string(&mut reader);
        message.set_sender_id(&sender_id);

        let receivers_count = reader.read_int() as usize;
        let mut receivers = Vec::with_capacity(receivers_count);
        for _ in 0..receivers_count {
            receivers.push(MusUtil::read_even_padded_string(&mut reader));
        }
        message.set_receivers(receivers);

        if message.get_subject() == "Logon" {
            // Read in remaining data
            let tmp_bytes = reader.read_bytes(reader.readable_bytes());
            let content = String::from_utf8_lossy(&tmp_bytes).to_string();

            // Set fields
            message.set_content_type(MusTypes::STRING);
            message.set_content_string(&content);
        } else {
            message.set_content_type(reader.read_short());

            if message.get_content_type() == MusTypes::INTEGER {
                message.set_content_int(reader.read_int());
            } else if message.get_content_type() == MusTypes::STRING {
                let content = MusUtil::read_even_padded_string(&mut reader);
                message.set_content_string(&content);
            } else if message.get_content_type() == MusTypes::PROPLIST {
                message.set_content_prop_list(Some(MusUtil::read_prop_list(&mut reader)));
            }
        }

        Some((message, false))
    }
}
