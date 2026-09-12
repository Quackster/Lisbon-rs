//! Mirrors `org.alexdev.duckhttpd.server.connection.WebConnection` and its session /
//! cookie / form / request companions (external Java library).

use std::cell::RefCell;
use std::collections::HashMap;

use http::StatusCode;

use crate::duckhttpd::session;

/// Mirrors a value stored in the Java session map (`String` / `Boolean` / `Integer`).
#[derive(Clone, Debug, PartialEq)]
pub enum SessionValue {
    Str(String),
    Bool(bool),
    Int(i32),
}

/// Mirrors `org.alexdev.duckhttpd.queries.WebSession`.
pub struct WebSession {
    data: RefCell<HashMap<String, SessionValue>>,
}

impl WebSession {
    pub fn new() -> Self {
        Self::from_map(HashMap::new())
    }

    pub fn from_map(data: HashMap<String, SessionValue>) -> Self {
        Self {
            data: RefCell::new(data),
        }
    }

    /// Mirrors `getAttributes()`.
    pub fn to_map(&self) -> HashMap<String, SessionValue> {
        self.data.borrow().clone()
    }

    /// Mirrors `loadSessionData()`.
    pub fn load_map(&self, data: HashMap<String, SessionValue>) {
        let mut current = self.data.borrow_mut();
        current.clear();
        current.extend(data);
    }

    /// Mirrors `contains(String)`.
    pub fn contains(&self, key: &str) -> bool {
        self.data.borrow().contains_key(key)
    }

    /// Mirrors `getString(String)`.
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.data.borrow().get(key).and_then(|value| {
            Some(match value {
                SessionValue::Str(string) => string.clone(),
                SessionValue::Bool(boolean) => boolean.to_string(),
                SessionValue::Int(integer) => integer.to_string(),
            })
        })
    }

    /// Mirrors `getInt(String)`.
    pub fn get_int(&self, key: &str) -> i32 {
        self.get_int_or(key, 0)
    }

    /// Mirrors `getIntOrElse(String, int)`.
    pub fn get_int_or(&self, key: &str, default: i32) -> i32 {
        self.data
            .borrow()
            .get(key)
            .and_then(|value| {
                Some(match value {
                    SessionValue::Str(string) => string.parse().ok()?,
                    SessionValue::Bool(boolean) => if *boolean { 1 } else { 0 },
                    SessionValue::Int(integer) => *integer,
                })
            })
            .unwrap_or(default)
    }

    /// Mirrors `getLong(String)` (Java `getLongOrElse(String, long)`).
    pub fn get_long(&self, key: &str) -> i64 {
        self.get_long_or(key, 0)
    }

    /// Mirrors `getLongOrElse(String, long)`.
    pub fn get_long_or(&self, key: &str, default: i64) -> i64 {
        self.data
            .borrow()
            .get(key)
            .and_then(|value| {
                Some(match value {
                    SessionValue::Str(string) => string.parse().ok()?,
                    SessionValue::Bool(boolean) => if *boolean { 1 } else { 0 },
                    SessionValue::Int(integer) => *integer as i64,
                })
            })
            .unwrap_or(default)
    }

    /// Mirrors `getBoolean(String)`.
    pub fn get_boolean(&self, key: &str) -> bool {
        self.data
            .borrow()
            .get(key)
            .and_then(|value| {
                Some(match value {
                    SessionValue::Bool(boolean) => *boolean,
                    SessionValue::Str(string) => string == "true",
                    SessionValue::Int(integer) => *integer != 0,
                })
            })
            .unwrap_or(false)
    }

    /// Mirrors `set(String, Object)`.
    pub fn set(&self, key: &str, value: SessionValue) {
        self.data.borrow_mut().insert(key.to_string(), value);
    }

    /// Mirrors `delete(String)`.
    pub fn delete(&self, key: &str) {
        self.data.borrow_mut().remove(key);
    }
}

/// Mirrors `org.alexdev.duckhttpd.queries.WebCookies`.
pub struct WebCookieStore {
    data: RefCell<HashMap<String, String>>,
    set_cookies: RefCell<Vec<(String, String, i64)>>,
}

impl WebCookieStore {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            set_cookies: RefCell::new(Vec::new()),
        }
    }

    pub fn set_request_cookie(&self, name: &str, value: &str) {
        self.data
            .borrow_mut()
            .insert(name.to_ascii_lowercase(), value.to_string());
    }

    /// Mirrors `set(String, String)` and `set(String, String, long, TimeUnit)`.
    pub fn set(&self, name: &str, value: &str, expiry_seconds: i64) {
        let mut cookies = self.set_cookies.borrow_mut();

        cookies.retain(|(existing, _, _)| !existing.eq_ignore_ascii_case(name));
        cookies.push((name.to_string(), value.to_string(), expiry_seconds));
    }

    /// Mirrors `get(String)`.
    pub fn get(&self, name: &str) -> Option<String> {
        self.data
            .borrow()
            .get(name.to_ascii_lowercase().as_str())
            .cloned()
    }

    /// Mirrors `exists(String)`.
    pub fn exists(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Mirrors `encodeCookies(HttpResponse)`.
    pub fn take_set_cookies(&self) -> Vec<String> {
        let cookies: Vec<String> = self
            .set_cookies
            .borrow()
            .iter()
            .map(|(name, value, expiry)| {
                if value.is_empty() && *expiry == 0 {
                    format!("{name}={value}; Path=/; HttpOnly; Max-Age=0")
                } else if *expiry > 0 {
                    format!("{name}={value}; Path=/; HttpOnly; Max-Age={expiry}")
                } else {
                    format!("{name}={value}; Path=/; HttpOnly")
                }
            })
            .collect();

        self.set_cookies.borrow_mut().clear();

        cookies
    }
}

/// Mirrors `org.alexdev.duckhttpd.queries.WebQuery`.
pub struct WebFormData {
    data: RefCell<HashMap<String, Vec<String>>>,
}

impl WebFormData {
    pub fn new() -> Self {
        Self::from_query_string("")
    }

    /// Mirrors the `WebQuery(String)` constructor.
    pub fn from_query_string(query_data: &str) -> Self {
        let mut data = HashMap::new();
        let query = query_data.trim_start_matches('/');
        let query = query
            .split('?')
            .last()
            .unwrap_or_default();

        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }

            let (key, value) = match pair.split_once('=') {
                Some((key, value)) => (key, value),
                None => (pair, ""),
            };

            if key.is_empty() {
                continue;
            }

            let key = unescape_html4(&urlencoding::decode(key).unwrap_or_default());
            let value = unescape_html4(&urlencoding::decode(value).unwrap_or_default());

            data.entry(key).or_insert_with(Vec::new).push(value);
        }

        Self {
            data: RefCell::new(data),
        }
    }

    /// Mirrors `contains(String)`.
    pub fn contains(&self, name: &str) -> bool {
        self.data.borrow().contains_key(name)
    }

    /// Mirrors `getString(String)`.
    pub fn get_string(&self, name: &str) -> Option<String> {
        self.data.borrow().get(name)?.first().cloned()
    }

    /// Mirrors `getInt(String)` (Java throws on a missing or unparseable value).
    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.data.borrow().get(name)?.first()?.parse().ok()
    }

    /// Mirrors `getBoolean(String)`.
    pub fn get_boolean(&self, name: &str) -> bool {
        self.data
            .borrow()
            .get(name)
            .and_then(|values| values.first())
            .map(|value| value == "true")
            .unwrap_or(false)
    }

    /// Mirrors `queries()`.
    pub fn queries(&self) -> Vec<String> {
        self.data.borrow().keys().cloned().collect()
    }

    /// Mirrors `getValues()`.
    pub fn get_values(&self) -> HashMap<String, String> {
        self.data
            .borrow()
            .iter()
            .filter(|(_, values)| !values.is_empty())
            .map(|(key, values)| (key.clone(), values.first().cloned().unwrap_or_default()))
            .collect()
    }

    /// Mirrors `getArray(String)` (Java returns a `List<String>`).
    pub fn get_array(&self, name: &str) -> Vec<String> {
        self.data.borrow().get(name).cloned().unwrap_or_default()
    }
}

fn unescape_html4(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&#x27;", "'")
        .replace("&#x2F;", "/")
        .replace("&#x2f;", "/")
}

/// Mirrors the Java request headers (`io.netty.handler.codec.http.HttpHeaders`).
/// Port note: the Java `HttpHeaders` multimap is collapsed to first-value entries.
pub struct RequestHeaders {
    data: RefCell<HashMap<String, String>>,
}

impl RequestHeaders {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, name: &str, value: &str) {
        if self.data.borrow().contains_key(&name.to_ascii_lowercase()) {
            return;
        }

        self.data
            .borrow_mut()
            .insert(name.to_ascii_lowercase(), value.to_string());
    }

    /// Mirrors `isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }

    /// Mirrors `get(CharSequence)`.
    pub fn get(&self, name: &str) -> Option<String> {
        self.data
            .borrow()
            .get(name.to_ascii_lowercase().as_str())
            .cloned()
    }

    /// Mirrors `contains(CharSequence)`.
    pub fn contains(&self, name: &str) -> bool {
        self.data
            .borrow()
            .contains_key(&name.to_ascii_lowercase())
    }
}

/// Mirrors `org.alexdev.duckhttpd.server.connection.WebRequest`.
pub struct WebRequest {
    uri: RefCell<String>,
    method: RefCell<String>,
    headers: RequestHeaders,
}

impl WebRequest {
    pub fn new() -> Self {
        Self {
            uri: RefCell::new("/".to_string()),
            method: RefCell::new("GET".to_string()),
            headers: RequestHeaders::new(),
        }
    }

    /// Mirrors `uri()`.
    pub fn uri(&self) -> String {
        self.uri.borrow().clone()
    }

    /// Mirrors the `FullHttpRequest` method of the Java request.
    pub fn method(&self) -> String {
        self.method.borrow().clone()
    }

    /// Mirrors `headers()`.
    pub fn headers(&self) -> &RequestHeaders {
        &self.headers
    }
}

/// Mirrors `org.alexdev.duckhttpd.server.connection.WebConnection`.
pub struct WebConnection {
    session: WebSession,
    cookies: WebCookieStore,
    post: WebFormData,
    get: WebFormData,
    request: WebRequest,
    headers: RefCell<Vec<(String, String)>>,
    route_request: RefCell<String>,
    matches: RefCell<Vec<String>>,
    request_handled: RefCell<bool>,
    response: RefCell<Option<super::Response>>,
    session_id: RefCell<Option<String>>,
    ip_address: RefCell<String>,
}

impl WebConnection {
    /// Mirrors the `WebConnection(Channel, FullHttpRequest)` constructor.
    pub fn from_request<B: AsRef<[u8]>>(
        request: &http::Request<B>,
        peer_ip: Option<String>,
    ) -> Self {
        let (path, query) = match request.uri().path_and_query() {
            Some(path_and_query) => (
                path_and_query.as_str().to_string(),
                path_and_query.query().unwrap_or("").to_string(),
            ),
            None => (request.uri().path().to_string(), String::new()),
        };

        let mut uri = path;

        if !query.is_empty() {
            uri.push('?');
            uri.push_str(&query);
        }

        let headers = RequestHeaders::new();

        for (name, value) in request.headers().iter() {
            if let Ok(value) = value.to_str() {
                headers.insert(name.as_str(), value);
            }
        }

        let request_content = String::from_utf8_lossy(request.body().as_ref());

        let mut ip_address = String::new();

        for header in ["X-Real-IP", "X-Forwarded-For", "HTTP_CF_CONNECTING_IP", "CF-Connecting-IP"]
        {
            if let Some(value) = headers.get(header) {
                ip_address = value;
                break;
            }
        }

        if ip_address.is_empty() {
            ip_address = peer_ip.unwrap_or_default();
        }

        let connection = Self {
            session: WebSession::new(),
            cookies: WebCookieStore::new(),
            post: WebFormData::from_query_string(&request_content),
            get: WebFormData::from_query_string(&query),
            request: WebRequest::new(),
            headers: RefCell::new(Vec::new()),
            route_request: RefCell::new(String::new()),
            matches: RefCell::new(Vec::new()),
            request_handled: RefCell::new(false),
            response: RefCell::new(None),
            session_id: RefCell::new(None),
            ip_address: RefCell::new(ip_address),
        };

        *connection.request.uri.borrow_mut() = uri;
        *connection.request.method.borrow_mut() = request.method().to_string();

        if let Some(cookie_header) = headers.get("Cookie") {
            for pair in cookie_header.split(',') {
                if let Some((name, value)) = pair.trim().split_once('=') {
                    connection
                        .cookies
                        .set_request_cookie(name.trim(), value.trim());
                }
            }
        }

        connection.validate_session();

        connection
    }

    /// Mirrors `validateSession()`.
    pub fn validate_session(&self) {
        let (fingerprint, data) = session::CookieSessionManager::get_instance().get_session(self);

        *self.session_id.borrow_mut() = Some(fingerprint.clone());
        self.cookies
            .set(session::HTTPSESSID, &fingerprint, 0);
        self.session.load_map(data);
    }

    /// Mirrors `WebSession.saveSessionData()` (deferred to the end of the request).
    pub fn save_session(&self) {
        if let Some(fingerprint) = self.session_id.borrow().clone() {
            session::CookieSessionManager::get_instance()
                .save_session(&fingerprint, self.session().to_map());
        }
    }

    /// Mirrors `session()`.
    pub fn session(&self) -> &WebSession {
        &self.session
    }

    /// Mirrors `cookies()`.
    pub fn cookies(&self) -> &WebCookieStore {
        &self.cookies
    }

    /// Mirrors `post()`.
    pub fn post(&self) -> &WebFormData {
        &self.post
    }

    /// Mirrors `get()`.
    pub fn get(&self) -> &WebFormData {
        &self.get
    }

    /// Mirrors `isRequestHandled()`.
    pub fn is_request_handled(&self) -> bool {
        *self.request_handled.borrow()
    }

    /// Mirrors `setRequestHandled(boolean)`.
    pub fn set_request_handled(&self, handled: bool) {
        *self.request_handled.borrow_mut() = handled;
    }

    /// Mirrors `send(String)`.
    pub fn send_string(&self, body: impl Into<String>) {
        self.send(super::ResponseBuilder::create(body));
    }

    /// Mirrors `movedPermanently(String)`.
    pub fn moved_permanently(&self, target_url: &str) {
        self.redirect_with_status(target_url, StatusCode::MOVED_PERMANENTLY);
    }

    /// Mirrors `redirect(String)`.
    pub fn redirect(&self, target_url: &str) {
        self.redirect_with_status(target_url, StatusCode::FOUND);
    }

    fn redirect_with_status(&self, target_url: &str, status: StatusCode) {
        let mut response = self
            .response
            .borrow()
            .clone()
            .unwrap_or_else(|| super::ResponseBuilder::create(""));

        response.status = status;
        response
            .headers
            .push(("Location".to_string(), target_url.to_string()));
        *self.response.borrow_mut() = Some(response);
    }

    /// Mirrors `getIpAddress()`.
    pub fn get_ip_address(&self) -> String {
        self.ip_address.borrow().clone()
    }

    /// Mirrors `request()`.
    pub fn request(&self) -> &WebRequest {
        &self.request
    }

    /// Mirrors `headers()` (map view of the pending response headers).
    pub fn headers(&self) -> Vec<(String, String)> {
        self.headers.borrow().clone()
    }

    /// Mirrors `headers().clear()`.
    pub fn clear_headers(&self) {
        self.headers.borrow_mut().clear();
    }

    /// Mirrors `headers().put(String, String)`.
    pub fn set_header(&self, name: &str, value: &str) {
        let mut headers = self.headers.borrow_mut();

        if let Some(entry) = headers.iter_mut().find(|(existing, _)| existing.eq_ignore_ascii_case(name)) {
            entry.1 = value.to_string();
        } else {
            headers.push((name.to_string(), value.to_string()));
        }
    }

    /// Mirrors `send(FullHttpResponse)`.
    pub fn send(&self, response: super::Response) {
        *self.response.borrow_mut() = Some(response);
    }

    /// Mirrors `response() != null`.
    pub fn has_response(&self) -> bool {
        self.response.borrow().is_some()
    }

    /// Mirrors `response()`.
    pub fn take_response(&self) -> Option<super::Response> {
        self.response.borrow_mut().take()
    }

    /// Mirrors `getMatches()`.
    pub fn get_matches(&self) -> Vec<String> {
        self.matches.borrow().clone()
    }

    /// Mirrors `setWildcardMatches(List)`.
    pub fn set_wildcard_matches(&self, matches: &[String]) {
        *self.matches.borrow_mut() = matches.to_vec();
    }

    /// Mirrors `getRouteRequest()`.
    pub fn get_route_request(&self) -> String {
        self.route_request.borrow().clone()
    }

    /// Mirrors `setRouteRequest(String)`.
    pub fn set_route_request(&self, route_request: &str) {
        *self.route_request.borrow_mut() = route_request.to_string();
    }

    /// Mirrors `template(String)`.
    pub fn template(&self, view: &str) -> crate::template::twig_template::TwigTemplate<'_> {
        let mut template = crate::template::twig_template::TwigTemplate::new(Some(self));
        template.start(view);
        template
    }
}
