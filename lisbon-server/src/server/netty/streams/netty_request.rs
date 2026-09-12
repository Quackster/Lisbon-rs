//! Mirrors `net.h4bbo.lisbon.server.netty.streams.NettyRequest`.
//!
//! Java reads from a Netty `ByteBuf`; here we model the buffer as a
//! `Vec<u8>` with a read cursor.

use crate::util::encoding::base64_encoding::Base64Encoding;
use crate::util::encoding::vl64_encoding::VL64Encoding;

pub struct NettyRequest {
    buffer: Vec<u8>,
    pos: usize,
    header: String,
    header_id: i32,
}

impl NettyRequest {
    /// Mirrors the constructor — reads the 2-byte header and decodes the id.
    pub fn new(buffer: Vec<u8>) -> Self {
        let b0 = buffer[0];
        let b1 = buffer.get(1).copied().unwrap_or(0);
        let header_bytes = [b0, b1];
        let header = String::from_utf8_lossy(&header_bytes).to_string();
        let header_id = Base64Encoding::decode(&header_bytes);

        Self {
            buffer,
            pos: 2,
            header,
            header_id,
        }
    }

    /// Mirrors `readInt`.
    pub fn read_int(&mut self) -> i32 {
        let remaining = self.remaining_bytes();
        let length = ((remaining[0] >> 3) & 7) as usize;
        let value = VL64Encoding::decode(&remaining);
        self.read_bytes(length);
        value
    }

    /// Mirrors `readBase64`.
    pub fn read_base64(&mut self) -> i32 {
        let b0 = *self.buffer.get(self.pos).unwrap_or(&0);
        let b1 = *self.buffer.get(self.pos + 1).unwrap_or(&0);
        self.pos += 2;
        Base64Encoding::decode(&[b0, b1])
    }

    /// Mirrors `readBoolean`.
    pub fn read_boolean(&mut self) -> bool {
        self.read_int() == 1
    }

    /// Mirrors `readString`.
    pub fn read_string(&mut self) -> String {
        let length = self.read_base64() as usize;
        let data = self.read_bytes(length);
        String::from_utf8_lossy(&data).to_string()
    }

    /// Mirrors `readBytes(int)`.
    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let start = self.pos;
        let end = (self.pos + len).min(self.buffer.len());
        let out = self.buffer[start..end].to_vec();
        self.pos = end;
        out
    }

    /// Mirrors `remainingBytes`.
    pub fn remaining_bytes(&self) -> Vec<u8> {
        self.buffer[self.pos..].to_vec()
    }

    /// Mirrors `contents`.
    pub fn contents(&self) -> Option<String> {
        Some(String::from_utf8_lossy(&self.remaining_bytes()).to_string())
    }

    /// Mirrors `getMessageBody`.
    pub fn get_message_body(&self) -> String {
        let mut console_text = String::from_utf8_lossy(&self.buffer).to_string();
        for i in 0..14 {
            let ch = i as u8 as char;
            console_text = console_text.replace(ch, &format!("{{{}}}", i));
        }
        format!("{}{}", self.header, console_text)
    }

    /// Mirrors `getHeader`.
    pub fn get_header(&self) -> &str {
        &self.header
    }

    /// Mirrors `getHeaderId`.
    pub fn get_header_id(&self) -> i32 {
        self.header_id
    }

    /// Mirrors `dispose`.
    pub fn dispose(&mut self) {
        self.buffer.clear();
    }
}
