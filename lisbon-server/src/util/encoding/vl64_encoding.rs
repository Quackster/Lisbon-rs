//! Mirrors `net.h4bbo.lisbon.util.encoding.VL64Encoding`.

pub struct VL64Encoding;

impl VL64Encoding {
    pub const NEGATIVE: u8 = 72;
    pub const POSITIVE: u8 = 73;
    pub const MAX_INTEGER_BYTE_AMOUNT: usize = 6;

    /// Mirrors `encode(int)`.
    pub fn encode(i: i32) -> Vec<u8> {
        let mut wf = [0u8; 6];
        let mut pos = 0usize;
        let mut num_bytes = 1usize;
        let start_pos = pos;
        let negative_mask = if i >= 0 { 0 } else { 4 };

        // `unsigned_abs` avoids the `i32::MIN` panic and yields the magnitude.
        let mut i = (i as i64).unsigned_abs();

        wf[pos] = (64 + (i & 3)) as u8;
        pos += 1;

        i >>= 2;
        while i != 0 {
            num_bytes += 1;
            wf[pos] = (64 + (i & 0x3f)) as u8;
            pos += 1;
            i >>= 6;
        }

        wf[start_pos] |= (num_bytes as u8) << 3 | negative_mask;

        wf[..num_bytes].to_vec()
    }

    /// Mirrors `decode(byte[])`.
    pub fn decode(bz_data: &[u8]) -> i32 {
        let mut pos = 0usize;

        let negative = (bz_data[pos] & 4) == 4;
        let total_bytes = (bz_data[pos] >> 3) & 7;

        let mut v: i32 = bz_data[pos] as i32 & 3;
        pos += 1;

        let mut shift_amount = 2;

        for b in 1..total_bytes {
            v |= ((bz_data[pos] as i32) & 0x3f) << shift_amount;
            shift_amount = 2 + 6 * b;
            pos += 1;
        }

        if negative {
            v = v.wrapping_mul(-1);
        }

        v
    }
}
