//! Mirrors `net.h4bbo.lisbon.server.rcon.codec.RconNetworkDecoder`.
//!
//! Java extends `ByteToMessageDecoder`, reading a 4-byte (big-endian) length
//! prefix followed by `length` body bytes. Here the framing is folded into
//! `decode`, which pulls a whole frame out of the accumulated `Vec<u8>`. The
//! Java `clear` / `tryRelease` helpers only manage `ByteBuf` ref-counting; in
//! Rust the frame slice is owned and dropped automatically, so they have no
//! direct equivalent.

use std::collections::HashMap;

use crate::server::rcon::messages::rcon_message::RconMessage;

/// A cursor over an immutable byte slice (mirrors the read view of a
/// `ByteBuf`).
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Mirrors `ByteBuf#readableBytes`.
    fn readable_bytes(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    /// Mirrors `ByteBuf#readInt` (big-endian).
    fn read_int(&mut self) -> i32 {
        if self.readable_bytes() < 4 {
            return 0;
        }
        let a = self.data[self.pos] as i32;
        let b = self.data[self.pos + 1] as i32;
        let c = self.data[self.pos + 2] as i32;
        let d = self.data[self.pos + 3] as i32;
        self.pos += 4;
        (a << 24) | (b << 16) | (c << 8) | d
    }

    /// Mirrors `RconNetworkDecoder#readBytes(ByteBuf, int)`.
    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let take = len.min(self.readable_bytes());
        let out = self.data[self.pos..self.pos + take].to_vec();
        self.pos += take;
        out
    }
}

/// The RCON frame decoder.
pub struct RconNetworkDecoder;

impl RconNetworkDecoder {
    /// Mirrors `decode(ChannelHandlerContext, ByteBuf, List<Object>)`.
    ///
    /// Returns `Some(message)` when a full frame was decoded, or `None` when
    /// more bytes are required (mirroring `buffer.resetReaderIndex()`).
    pub fn decode(buf: &mut Vec<u8>) -> Option<RconMessage> {
        if buf.len() < 4 {
            return None;
        }

        let length = i32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if length < 0 {
            return None;
        }

        let total = 4 + length as usize;
        if buf.len() < total {
            return None;
        }

        let body = buf[4..total].to_vec();
        buf.drain(..total);

        let mut reader = Reader::new(&body);
        let header = Self::read_string(&mut reader)?;

        let parameter_count = reader.read_int() as usize;
        let mut parameters = HashMap::with_capacity(parameter_count);

        for _ in 0..parameter_count {
            let key = Self::read_string(&mut reader)?;
            let value = Self::read_string(&mut reader)?;
            parameters.insert(key, value);
        }

        Some(RconMessage::new(&header, parameters))
    }

    /// Mirrors `readString(ByteBuf)`.
    fn read_string(reader: &mut Reader) -> Option<String> {
        let length = reader.read_int();
        if length < 0 || (length as usize) > reader.readable_bytes() {
            return None;
        }

        let data = reader.read_bytes(length as usize);
        Some(String::from_utf8_lossy(&data).to_string())
    }
}
