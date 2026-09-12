//! Mirrors `net.h4bbo.lisbon.server.netty.streams.NettyResponse`.
//!
//! Java writes into a Netty `ByteBuf`; here we accumulate into a `Vec<u8>`.

use crate::util::encoding::base64_encoding::Base64Encoding;
use crate::util::encoding::vl64_encoding::VL64Encoding;

pub struct NettyResponse {
    id: i16,
    buffer: Vec<u8>,
    finalised: bool,
}

impl NettyResponse {
    /// Mirrors the constructor — writes the 2-byte header.
    pub fn new(header: i16) -> Self {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&Base64Encoding::encode(header as i32, 2));
        Self {
            id: header,
            buffer,
            finalised: false,
        }
    }

    /// Mirrors `write(Object)`.
    pub fn write<T: std::fmt::Display>(&mut self, obj: T) {
        let s = format!("{}", obj);
        self.buffer.extend_from_slice(s.as_bytes());
    }

    /// Mirrors `writeString(Object)`.
    pub fn write_string<T: std::fmt::Display>(&mut self, obj: T) {
        let s = format!("{}", obj);
        self.buffer.extend_from_slice(s.as_bytes());
        self.buffer.push(2);
    }

    /// Mirrors `writeInt(Integer)`.
    pub fn write_int(&mut self, number: i32) {
        self.buffer.extend_from_slice(&VL64Encoding::encode(number));
    }

    /// Mirrors `writeKeyValue(Object, Object)`.
    pub fn write_key_value<K: std::fmt::Display, V: std::fmt::Display>(
        &mut self,
        key: K,
        value: V,
    ) {
        self.buffer.extend_from_slice(format!("{}", key).as_bytes());
        self.buffer.extend_from_slice(b":");
        self.buffer.extend_from_slice(format!("{}", value).as_bytes());
        self.buffer.push(13);
    }

    /// Mirrors `writeValue(Object, Object)`.
    pub fn write_value<K: std::fmt::Display, V: std::fmt::Display>(
        &mut self,
        key: K,
        value: V,
    ) {
        self.buffer.extend_from_slice(format!("{}", key).as_bytes());
        self.buffer.extend_from_slice(b"=");
        self.buffer.extend_from_slice(format!("{}", value).as_bytes());
        self.buffer.push(13);
    }

    /// Mirrors `writeDelimeter(Object, Object)`.
    pub fn write_delimeter<K: std::fmt::Display, V: std::fmt::Display>(
        &mut self,
        key: K,
        value: V,
    ) {
        self.buffer.extend_from_slice(format!("{}", key).as_bytes());
        self.buffer.extend_from_slice(format!("{}", value).as_bytes());
    }

    /// Mirrors `writeBool(Boolean)`.
    pub fn write_bool(&mut self, obj: bool) {
        self.write_int(if obj { 1 } else { 0 });
    }

    /// Mirrors `getBodyString`.
    pub fn get_body_string(&self) -> String {
        let mut str = String::from_utf8_lossy(&self.buffer).to_string();
        for i in 0..14 {
            let ch = i as u8 as char;
            str = str.replace(ch, &format!("{{{}}}", i));
        }
        str
    }

    /// Mirrors `isFinalised`.
    pub fn is_finalised(&self) -> bool {
        self.finalised
    }

    /// Mirrors `setFinalised`.
    pub fn set_finalised(&mut self, finalised: bool) {
        self.finalised = finalised;
    }

    /// Mirrors `getHeader`.
    pub fn get_header(&self) -> i32 {
        self.id as i32
    }

    /// Returns the accumulated buffer (for sending over the wire).
    pub fn take_buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}
