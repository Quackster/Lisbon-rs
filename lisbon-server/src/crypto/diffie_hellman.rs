//! Mirrors `net.h4bbo.lisbon.crypto.DiffieHellman`.
//!
//! Java uses `java.math.BigInteger` + `SecureRandom`; here we use
//! `num_bigint::BigUint` + `rand`.

use num_bigint::BigUint;
use num_traits::One;
use rand::Rng;

const INIT_PRIME_HEX: &str = "A8EA077D4943CC98E53C21F5F7C7A0DB8BCE7506F8361A7C1690392F2B090C96EE8BC67BAA0DCB7183F16401F5CB838E3B6EE86B9EF2E5D0F3C49D4DC4EDC2B9";
const INIT_GENERATOR: u64 = 5;
const INIT_PRIVATE_CHARS: &str = "012345679abcdef";
const INIT_COMPAT_SUFFIX_CHARS: &str = "GHIJKLMNOPQRSTUVWXYZ";
const INIT_COMPAT_MIN_PADDING: usize = 11;
const INIT_COMPAT_PADDING_VARIATION: usize = 8;

pub struct DiffieHellman {
    prime: BigUint,
    private_key: BigUint,
    public_key: BigUint,
}

impl DiffieHellman {
    fn new(prime: BigUint, private_key: BigUint, public_key: BigUint) -> Self {
        Self {
            prime,
            private_key,
            public_key,
        }
    }

    /// Mirrors `init()`.
    pub fn init() -> Self {
        let prime = BigUint::parse_bytes(INIT_PRIME_HEX.as_bytes(), 16).unwrap();
        let generator = BigUint::from(INIT_GENERATOR);
        Self::generate(&prime, &generator, 40, INIT_PRIVATE_CHARS, 72, 4)
    }

    /// Mirrors `initCompatibilityPublicKeyHex()`.
    pub fn init_compatibility_public_key_hex() -> String {
        let zero_count =
            INIT_COMPAT_MIN_PADDING + rand::thread_rng().gen_range(0..INIT_COMPAT_PADDING_VARIATION);
        let suffix: Vec<char> = INIT_COMPAT_SUFFIX_CHARS.chars().collect();
        let suffix = suffix[rand::thread_rng().gen_range(0..suffix.len())];

        let mut value = String::with_capacity(zero_count + 2);
        for _ in 0..zero_count {
            value.push('0');
        }
        value.push('1');
        value.push(suffix);
        value
    }

    /// Mirrors `getPublicKeyHex()`.
    pub fn get_public_key_hex(&self) -> String {
        self.public_key.to_str_radix(16).to_uppercase()
    }

    /// Mirrors `getPrivateKeyHex()`.
    pub fn get_private_key_hex(&self) -> String {
        self.private_key.to_str_radix(16).to_uppercase()
    }

    /// Mirrors `computeSharedSecret(String)`.
    pub fn compute_shared_secret(&self, client_public_key_hex: &str) -> Result<Vec<u8>, String> {
        let client_public = Self::parse_public_key_hex(client_public_key_hex)?;
        let shared_secret = client_public.modpow(&self.private_key, &self.prime);
        let mut shared_hex = shared_secret.to_str_radix(16);

        if shared_hex.len() % 2 != 0 {
            shared_hex = format!("0{}", shared_hex);
        }

        let mut result = vec![0u8; shared_hex.len() / 2];
        for i in 0..result.len() {
            result[i] = u8::from_str_radix(&shared_hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| e.to_string())?;
        }
        Ok(result)
    }

    /// Mirrors `parsePublicKeyHex(String)`.
    pub fn parse_public_key_hex(value: &str) -> Result<BigUint, String> {
        if value.is_empty() || value.trim().is_empty() {
            return Err("Public key cannot be blank".to_string());
        }

        if let Some(v) = BigUint::parse_bytes(value.as_bytes(), 16) {
            return Ok(v);
        }

        let hex_digits: String = value.chars().filter(|c| c.is_digit(16)).collect();
        if hex_digits.is_empty() {
            return Err(format!(
                "Compatibility key does not contain hex digits: {}",
                value
            ));
        }

        Ok(BigUint::parse_bytes(hex_digits.as_bytes(), 16)
            .expect("filtered hex digits always parse"))
    }

    /// Mirrors `generate(...)`.
    fn generate(
        prime: &BigUint,
        generator: &BigUint,
        private_hex_bytes: usize,
        alphabet: &str,
        minimum_public_length: usize,
        max_attempts: usize,
    ) -> Self {
        let mut private_key = BigUint::one();
        let mut public_key = BigUint::one();

        for _ in 0..max_attempts {
            let private_hex = Self::random_hex(private_hex_bytes * 2, alphabet);
            private_key = BigUint::parse_bytes(private_hex.as_bytes(), 16)
                .unwrap_or_else(BigUint::one);
            public_key = generator.modpow(&private_key, prime);
            if public_key.to_str_radix(16).len() >= minimum_public_length {
                break;
            }
        }

        Self::new(prime.clone(), private_key, public_key)
    }

    /// Mirrors `randomHex(int, String)`.
    fn random_hex(length: usize, alphabet: &str) -> String {
        let alphabet: Vec<char> = alphabet.chars().collect();
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| alphabet[rng.gen_range(0..alphabet.len())])
            .collect()
    }
}
