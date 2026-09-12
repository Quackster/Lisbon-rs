//! Mirrors `net.h4bbo.lisbon.util.encoding.Base64Encoding`.

pub struct Base64Encoding;

impl Base64Encoding {
    pub const NEGATIVE: u8 = 64;
    pub const POSITIVE: u8 = 65;

    /// Mirrors `encode(int, int numBytes)`.
    pub fn encode(i: i32, num_bytes: usize) -> Vec<u8> {
        let mut bz_res = vec![0u8; num_bytes];
        for j in 1..=num_bytes {
            let k = (num_bytes - j) * 6;
            bz_res[j - 1] = (0x40 + ((i >> k) & 0x3f)) as u8;
        }
        bz_res
    }

    /// Mirrors `decode(byte[])`.
    pub fn decode(bz_data: &[u8]) -> i32 {
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        for k in (0..bz_data.len()).rev() {
            let x = (bz_data[k] as i64) - 0x40;
            let x = if j > 0 { x * 64i64.pow(j as u32) } else { x };
            i += x;
            j += 1;
        }
        i as i32
    }
}
