//! Mirrors `org.alexdev.duckhttpd.session.CookieSessionManager` and `CookieSession`
//! (external Java library).
//!
//! Port note: the Java session manager persists session files under `tmp/` when
//! `saveSessions` is set; the port keeps session data in process memory only.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use crate::duckhttpd::web_connection::{SessionValue, WebConnection};

/// Mirrors `CookieSessionManager.HTTPSESSID`.
pub const HTTPSESSID: &str = "HTTPSESSID";

/// Mirrors `CookieSessionManager.EXPIRE_TIME` (`TimeUnit.HOURS.toMinutes(24)`).
pub const EXPIRE_TIME_MINUTES: i64 = 24 * 60;

/// Mirrors `org.alexdev.duckhttpd.session.CookieSessionManager`.
pub struct CookieSessionManager {
    sessions: Mutex<HashMap<String, (i64, HashMap<String, SessionValue>)>>,
}

impl CookieSessionManager {
    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static CookieSessionManager {
        static INSTANCE: OnceLock<CookieSessionManager> = OnceLock::new();

        INSTANCE.get_or_init(|| {
            CookieSessionManager {
                sessions: Mutex::new(HashMap::new()),
            }
        })
    }

    /// Mirrors `getSession(WebConnection)`.
    pub fn get_session(&self, connection: &WebConnection) -> (String, HashMap<String, SessionValue>) {
        self.clear_expired();

        if let Some(cookie) = connection.cookies().get(HTTPSESSID) {
            if !cookie.trim().is_empty() {
                let sessions = self.sessions.lock().unwrap();

                if let Some((_, data)) = sessions.get(&cookie) {
                    return (cookie, data.clone());
                }
            }
        }

        (Self::generate_fingerprint(), HashMap::new())
    }

    /// Mirrors `WebSession.saveSessionData()`.
    pub fn save_session(&self, fingerprint: &str, data: HashMap<String, SessionValue>) {
        if data.is_empty() {
            return;
        }

        let expire = Self::current_time_millis()
            + EXPIRE_TIME_MINUTES * 60_000;

        self.sessions
            .lock()
            .unwrap()
            .insert(fingerprint.to_string(), (expire, data));
    }

    fn clear_expired(&self) {
        let now = Self::current_time_millis();
        self.sessions
            .lock()
            .unwrap()
            .retain(|_, (expire, _)| *expire > now);
    }

    /// Mirrors `CookieSession.generateFingerprint()`.
    fn generate_fingerprint() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);

        hex::encode(bytes)
    }

    fn current_time_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }
}
