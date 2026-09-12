//! Mirrors `org.alexdev.http.util.XSSUtil`.

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::duckhttpd::web_connection::{SessionValue, WebConnection};

/// Mirrors `org.alexdev.http.util.XSSUtil`.
pub struct XssUtil;

impl XssUtil {
    pub const XSS_KEY: &'static str = "xssKey";
    pub const XSS_SEED: &'static str = "xssSeed";
    pub const XSS_REQUESTED: &'static str = "xssRequested";

    /// Mirrors `verifyKey(WebConnection, String)`.
    pub fn verify_key(connection: &WebConnection, verify_route_request: &str) -> bool {
        let session = connection.session();

        if !session.contains(Self::XSS_KEY)
            || !session.contains(Self::XSS_SEED)
            || !session.contains(Self::XSS_REQUESTED)
        {
            Self::clear(connection);
            return false;
        }

        let expected_route_request = session
            .get_string(Self::XSS_REQUESTED)
            .unwrap_or_default();

        if !verify_route_request.eq_ignore_ascii_case(&expected_route_request) {
            Self::clear(connection);
            return false;
        }

        let key = Self::random_int(session.get_int(Self::XSS_SEED));

        if key != session.get_int(Self::XSS_KEY) {
            Self::clear(connection);
            return false;
        }

        Self::clear(connection);
        true
    }

    /// Mirrors `createKey(WebConnection, String)`.
    pub fn create_key(connection: &WebConnection, expected_route_request: &str) {
        Self::clear(connection);

        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(i32::MIN..i32::MAX);
        let key = Self::random_int(seed);

        connection
            .session()
            .set(Self::XSS_KEY, SessionValue::Int(key));
        connection
            .session()
            .set(Self::XSS_SEED, SessionValue::Int(seed));
        connection.session().set(
            Self::XSS_REQUESTED,
            SessionValue::Str(expected_route_request.to_string()),
        );
    }

    /// Mirrors `clear(WebConnection)`.
    pub fn clear(connection: &WebConnection) {
        if connection.session().contains(Self::XSS_KEY)
            || connection.session().contains(Self::XSS_SEED)
            || connection.session().contains(Self::XSS_REQUESTED)
        {
            connection.session().delete(Self::XSS_SEED);
            connection.session().delete(Self::XSS_KEY);
            connection.session().delete(Self::XSS_REQUESTED);
        }
    }

    /// Mirrors `new Random(seed).nextInt()`.
    fn random_int(seed: i32) -> i32 {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        rng.gen_range(i32::MIN..i32::MAX)
    }
}
