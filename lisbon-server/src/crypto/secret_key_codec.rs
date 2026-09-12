//! Mirrors `net.h4bbo.lisbon.crypto.SecretKeyCodec`.

pub struct SecretKeyCodec;

impl SecretKeyCodec {
    /// Mirrors `secretDecode(String)`.
    pub fn secret_decode(encoded_key: Option<&str>) -> i32 {
        let encoded_key: Vec<char> = match encoded_key {
            Some(s) => s.chars().collect(),
            None => return 0,
        };

        if encoded_key.is_empty() {
            return 0;
        }

        let length = encoded_key.len();
        let adjusted = if length % 2 != 0 { length - 1 } else { length };

        let half = encoded_key.len() / 2;
        let table: Vec<char> = encoded_key[..half].to_vec();
        let key: Vec<char> = encoded_key[half..adjusted].to_vec();

        let mut checksum: i32 = 0;

        for (index, value) in key.iter().enumerate() {
            let offset = table
                .iter()
                .position(|c| c == value)
                .map(|p| p as i32)
                .unwrap_or(-1);

            let mut decoded = if offset >= 0 { offset } else { -1 };

            if decoded % 2 == 0 {
                decoded *= 2;
            }
            if index % 3 == 0 {
                decoded *= 3;
            }
            if decoded < 0 {
                decoded = (key.len() % 2) as i32;
            }

            checksum += decoded;
            checksum ^= decoded << ((index % 3) * 8);
        }

        checksum
    }
}
