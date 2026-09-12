//! Mirrors `net.h4bbo.lisbon.server.mus.streams.MusPropList`.

use crate::server::mus::streams::mus_types as MusTypes;
use crate::util::bit_util::BitUtil;

/// A fixed-size list of named values, mirroring the Java parallel arrays
/// (`symbols`, `dataTypes`, `data`). Unfilled slots are `None`.
pub struct MusPropList {
    entries: Vec<Option<(String, i16, Vec<u8>)>>,
}

impl MusPropList {
    /// Mirrors the `MusPropList(int)` constructor.
    pub fn new(length: usize) -> Self {
        Self {
            entries: vec![None; length],
        }
    }

    /// Mirrors `setPropAsBytes(String, short, byte[])`.
    pub fn set_prop_as_bytes(&mut self, symbol: &str, ty: i16, data: Vec<u8>) -> bool {
        for slot in self.entries.iter_mut() {
            if slot.is_none() {
                *slot = Some((symbol.to_string(), ty, data));
                return true;
            }
        }

        // No space
        false
    }

    /// Mirrors `setPropAsInt(String, int)`.
    pub fn set_prop_as_int(&mut self, symbol: &str, value: i32) {
        let data = BitUtil::int_to_bytes(value);
        self.set_prop_as_bytes(symbol, MusTypes::INTEGER, data.to_vec());
    }

    /// Mirrors `setPropAsString(String, String)`.
    pub fn set_prop_as_string(&mut self, symbol: &str, value: &str) {
        self.set_prop_as_bytes(symbol, MusTypes::STRING, value.as_bytes().to_vec());
    }

    /// Mirrors `getPropType(String)`.
    pub fn get_prop_type(&self, symbol: &str) -> i16 {
        for slot in &self.entries {
            if let Some((s, ty, _)) = slot {
                if s == symbol {
                    return *ty;
                }
            }
        }

        MusTypes::VOID
    }

    /// Mirrors `getPropAsBytes(String)`.
    pub fn get_prop_as_bytes(&self, symbol: &str) -> Vec<u8> {
        for slot in &self.entries {
            if let Some((s, _, data)) = slot {
                if s == symbol {
                    return data.clone();
                }
            }
        }

        Vec::new()
    }

    /// Mirrors `getPropAsInt(String)`.
    pub fn get_prop_as_int(&self, symbol: &str) -> i32 {
        let bytes = self.get_prop_as_bytes(symbol);
        if bytes.is_empty() {
            -1
        } else {
            BitUtil::bytes_to_int(&bytes)
        }
    }

    /// Mirrors `getPropAsString(String)`.
    pub fn get_prop_as_string(&self, symbol: &str) -> String {
        let bytes = self.get_prop_as_bytes(symbol);
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Mirrors `getSymbolAt(int)`.
    pub fn get_symbol_at(&self, slot: usize) -> Option<&str> {
        self.entries
            .get(slot)
            .and_then(|e| e.as_ref())
            .map(|(s, _, _)| s.as_str())
    }

    /// Mirrors `getDataTypeAt(int)`.
    pub fn get_data_type_at(&self, slot: usize) -> i16 {
        self.entries
            .get(slot)
            .and_then(|e| e.as_ref())
            .map(|(_, ty, _)| *ty)
            .unwrap_or(0)
    }

    /// Mirrors `getDataAt(int)`.
    pub fn get_data_at(&self, slot: usize) -> Vec<u8> {
        self.entries
            .get(slot)
            .and_then(|e| e.as_ref())
            .map(|(_, _, data)| data.clone())
            .unwrap_or_default()
    }

    /// Mirrors `length()`.
    pub fn length(&self) -> usize {
        self.entries.len()
    }
}
