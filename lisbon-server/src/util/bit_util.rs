//! Mirrors `net.h4bbo.lisbon.util.BitUtil`.

const NUMBER_OF_BITS_IN_A_BYTE: i32 = 8;
const MASK_TO_BYTE: i32 = 0xFF;

pub struct BitUtil;

impl BitUtil {
    /// Mirrors `intToBytes` (big-endian, 4 bytes).
    pub fn int_to_bytes(i: i32) -> [u8; 4] {
        let mut bytes = [0u8; 4];
        let mut i = i;
        bytes[3] = (i & MASK_TO_BYTE) as u8;
        i >>= NUMBER_OF_BITS_IN_A_BYTE;
        bytes[2] = (i & MASK_TO_BYTE) as u8;
        i >>= NUMBER_OF_BITS_IN_A_BYTE;
        bytes[1] = (i & MASK_TO_BYTE) as u8;
        i >>= NUMBER_OF_BITS_IN_A_BYTE;
        bytes[0] = (i & MASK_TO_BYTE) as u8;
        bytes
    }

    /// Mirrors `bytesToInt`.
    pub fn bytes_to_int(bytes: &[u8]) -> i32 {
        let a = bytes[0] as i32;
        let b = bytes[1] as i32;
        let c = bytes[2] as i32;
        let d = bytes[3] as i32;
        (d & MASK_TO_BYTE) | ((c & MASK_TO_BYTE) << 8) | ((b & MASK_TO_BYTE) << 16) | ((a & MASK_TO_BYTE) << 24)
    }
}
