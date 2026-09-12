//! Mirrors `net.h4bbo.lisbon.server.mus.MusUtil`.
//!
//! Java reads/writes a Netty `ByteBuf`; here the read side is modelled as a
//! [`Reader`] cursor over a byte slice and the write side as a `Vec<u8>`.
//! Integer/short reads and writes are big-endian (network order), matching
//! Netty `ByteBuf`.

use crate::server::mus::streams::mus_prop_list::MusPropList;
use crate::server::mus::streams::mus_types as MusTypes;

/// A cursor over an immutable byte slice (mirrors the read view of a
/// `ByteBuf`).
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Create a reader over the given bytes.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Mirrors `ByteBuf#readableBytes`.
    pub fn readable_bytes(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Mirrors `ByteBuf#readByte`.
    pub fn read_byte(&mut self) -> u8 {
        let b = self.data[self.pos];
        self.pos += 1;
        b
    }

    /// Mirrors `ByteBuf#readShort` (big-endian).
    pub fn read_short(&mut self) -> i16 {
        let hi = self.data[self.pos] as i16;
        let lo = self.data[self.pos + 1] as i16;
        self.pos += 2;
        (hi << 8) | lo
    }

    /// Mirrors `ByteBuf#readInt` (big-endian).
    pub fn read_int(&mut self) -> i32 {
        let a = self.data[self.pos] as i32;
        let b = self.data[self.pos + 1] as i32;
        let c = self.data[self.pos + 2] as i32;
        let d = self.data[self.pos + 3] as i32;
        self.pos += 4;
        (a << 24) | (b << 16) | (c << 8) | d
    }

    /// Mirrors `ByteBuf#readBytes(int)`.
    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let out = self.data[self.pos..self.pos + len].to_vec();
        self.pos += len;
        out
    }
}

pub struct MusUtil;

impl MusUtil {
    /// Mirrors `readEvenPaddedString(ByteBuf)`.
    pub fn read_even_padded_string(reader: &mut Reader) -> String {
        // String length
        let length = reader.read_int();
        if length <= 0 {
            return String::new();
        }

        // Actual string bytes
        let bytes = reader.read_bytes(length as usize);

        // Advance one byte if uneven
        if (length as usize) % 2 != 0 {
            reader.read_byte();
        }

        // Return the string
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Mirrors `writeEvenPaddedString(ByteBuf, String)`.
    pub fn write_even_padded_string(out: &mut Vec<u8>, str: &str) {
        // String length
        out.extend_from_slice(&(str.len() as i32).to_be_bytes());

        // Actual string bytes
        out.extend_from_slice(str.as_bytes());

        // Add a null byte if uneven
        if str.len() % 2 != 0 {
            out.push(0);
        }
    }

    /// Mirrors `readPropList(ByteBuf)`.
    pub fn read_prop_list(reader: &mut Reader) -> MusPropList {
        // Length of list
        let length = reader.read_int() as usize;

        // Allocate props
        let mut props = MusPropList::new(length);

        // Parse them
        for _ in 0..length {
            // Symbol type (always string)
            reader.read_short();

            // Symbol (key)
            let symbol = Self::read_even_padded_string(reader);

            // Data type
            let data_type = reader.read_short();

            // Data (value)
            let data_length = if data_type == MusTypes::INTEGER {
                4
            } else {
                reader.read_int() as usize
            };
            let data = reader.read_bytes(data_length);
            if data_length % 2 != 0 {
                reader.read_byte();
            }

            // Set prop
            props.set_prop_as_bytes(&symbol, data_type, data);
        }

        props
    }

    /// Mirrors `writePropList(ByteBuf, MusPropList)`.
    pub fn write_prop_list(out: &mut Vec<u8>, props: &MusPropList) {
        // Length
        out.extend_from_slice(&(props.length() as i32).to_be_bytes());

        // Serialize elements
        for i in 0..props.length() {
            // Symbol
            out.extend_from_slice(&MusTypes::SYMBOL.to_be_bytes());
            let symbol = props.get_symbol_at(i).unwrap_or("");
            Self::write_even_padded_string(out, symbol);

            // Value
            out.extend_from_slice(&props.get_data_type_at(i).to_be_bytes());
            let data = props.get_data_at(i);
            if props.get_data_type_at(i) != MusTypes::INTEGER {
                out.extend_from_slice(&(data.len() as i32).to_be_bytes());
            }
            out.extend_from_slice(&data);
            if data.len() % 2 != 0 {
                out.push(0);
            }
        }
    }
}
