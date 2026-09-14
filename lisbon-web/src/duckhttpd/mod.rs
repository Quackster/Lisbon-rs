//! Mirrors the `org.alexdev.duckhttpd` web server library (external Java dependency).

pub mod mime_type;
pub mod page_rules;
pub mod session;
pub mod web_connection;

pub use web_connection::{WebConnection, WebSession};

use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::HttpBody;
use axum::extract::ConnectInfo;
use http::StatusCode;
use serde::{Serialize, Serializer};

use crate::template::twig_template::TwigTemplate;

/// Mirrors `org.alexdev.duckhttpd.routes.RouteHandler`.
pub type RouteHandler =
    fn(&WebConnection) -> Result<(), Box<dyn std::error::Error>>;

/// Mirrors `org.alexdev.duckhttpd.routes.Route`.
#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub handler: RouteHandler,
}

/// Mirrors `org.alexdev.duckhttpd.routes.RouteManager`.
pub struct RouteManager;

lazy_static::lazy_static! {
    static ref ROUTES: Mutex<HashMap<String, Route>> = Mutex::new(HashMap::new());
}

impl RouteManager {
    /// Mirrors the `RouteManager` singleton.
    pub fn get_instance() -> &'static RouteManager {
        static INSTANCE: RouteManager = RouteManager;
        &INSTANCE
    }

    /// Mirrors `addRoute(String[], RouteHandler)`.
    pub fn add_route(paths: &[&str], handler: RouteHandler) {
        let mut routes = ROUTES.lock().unwrap();

        for path in paths {
            routes.insert(
                path.to_string(),
                Route {
                    path: path.to_string(),
                    handler,
                },
            );
        }
    }

    /// Mirrors `getRoutes()`.
    pub fn get_routes() -> Vec<Route> {
        ROUTES.lock().unwrap().values().cloned().collect()
    }

    /// Mirrors `RouteManager.getRoute(WebConnection, String)`.
    pub fn get_route(&self, connection: &WebConnection, uri: &str) -> Option<Route> {
        let uri = uri.split('?').next().unwrap_or("");
        connection.set_route_request(uri);

        let routes = ROUTES.lock().unwrap();

        if let Some(route) = routes.get(uri) {
            return Some(route.clone());
        }

        let mut candidates: Vec<&Route> = routes
            .values()
            .filter(|route| route.path.contains('*'))
            .collect();

        candidates.sort_by(|a, b| {
            a.path
                .len()
                .cmp(&b.path.len())
                .then_with(|| a.path.cmp(&b.path))
        });

        for route in candidates {
            let base_uri = &route.path[..route.path.find('*').unwrap()];

            if !uri.starts_with(base_uri) {
                continue;
            }

            let group = if Self::count_matches(&route.path, '*') > 1 {
                "(.*?)"
            } else {
                "(.*)"
            };
            let pattern = route.path.replace('*', group);

            let matched = regex::Regex::new(&pattern)
                .map(|regex| regex.is_match(uri))
                .unwrap_or(false);

            if matched {
                let matches = Self::get_wildcard_entries(&route.path, uri);

                if !matches.is_empty() {
                    connection.set_wildcard_matches(&matches);
                    return Some(route.clone());
                }
            }
        }

        None
    }

    /// Mirrors the per-request dispatch of
    /// `org.alexdev.duckhttpd.server.WebChannelHandler#channelRead0`.
    pub fn invoke(&self, connection: &WebConnection) {
        connection.set_request_handled(false);

        for prefix in Settings::get_instance().get_block_ipv4() {
            if connection.get_ip_address().starts_with(&prefix) {
                return;
            }
        }

        if let Some(referrer) = connection.request().headers().get("Referer") {
            let rules = page_rules::PageRules::get_instance();

            if let Some(rule) = rules.matches_rule(&referrer) {
                connection.moved_permanently(&rules.get_new_url(&rule, &referrer));
            }
        }

        let raw_route = self.get_route(connection, "");

        let mut new_uri = connection.request().uri();

        if let Ok(decoded) = urlencoding::decode(&new_uri) {
            new_uri = decoded.into_owned();
        }

        let route = self.get_route(connection, &new_uri);

        if let Some(raw) = raw_route {
            if route.is_some() {
                connection.set_request_handled(true);
            }

            self.handle_route(connection, raw);
        }

        if let Some(route) = route {
            if !connection.has_response() {
                if connection.request().uri().contains("//") {
                    connection.redirect(&new_uri);
                } else {
                    self.handle_route(connection, route);
                }
            }
        }

        if !connection.has_response() {
            if !Self::serve_static_file(connection) {
                if let Some(response) = Settings::get_instance()
                    .get_default_responses()
                    .get_response(StatusCode::NOT_FOUND, connection)
                {
                    connection.send(response);
                }
            }
        }

        connection.save_session();
    }

    fn handle_route(&self, connection: &WebConnection, route: Route) {
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| (route.handler)(connection)));

        match result {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                self.send_error_response(
                    connection,
                    Some(&WebException::Error(error.to_string())),
                );
            }
            Err(_) => {
                self.send_error_response(
                    connection,
                    Some(&WebException::Error("handler panicked".to_string())),
                );
            }
        }
    }

    fn send_error_response(&self, connection: &WebConnection, exception: Option<&WebException>) {
        if let Some(response) = Settings::get_instance()
            .get_default_responses()
            .get_error_response(connection, exception)
        {
            connection.send(response);
        }
    }

    fn count_matches(syntax: &str, needle: char) -> usize {
        syntax.matches(needle).count()
    }

    /// Mirrors `WebUtilities.getWildcardEntries(String, String)`.
    pub fn get_wildcard_entries(syntax: &str, input: &str) -> Vec<String> {
        let mut entries = Vec::new();
        let group = if Self::count_matches(syntax, '*') > 1 {
            "(.*?)"
        } else {
            "(.*)"
        };

        let mut compiled = syntax.replace('*', group);
        let mut input_with_slash: Option<String> = None;

        if compiled.ends_with(group) {
            compiled.push('/');

            if !input.ends_with('/') {
                let mut with_slash = input.to_string();
                with_slash.push('/');
                input_with_slash = Some(with_slash);
            }
        }

        let input = input_with_slash.as_deref().unwrap_or(input);

        let regex = match regex::Regex::new(&compiled) {
            Ok(regex) => regex,
            Err(_) => return entries,
        };

        for captures in regex.captures_iter(input) {
            for index in 1..regex.captures_len() {
                if let Some(group) = captures.get(index) {
                    entries.push(group.as_str().to_string());
                }
            }
        }

        entries
    }

    /// Mirrors `ResponseBuilder.create(WebConnection, FullHttpRequest)` (static
    /// site files from `Settings.getSiteDirectory()`).
    fn serve_static_file(connection: &WebConnection) -> bool {
        let uri = connection.request().uri();
        let normalized = uri.replace("//", "/");
        let file_uri = normalized.split('?').next().unwrap_or("");

        let file_uri = match urlencoding::decode(file_uri) {
            Ok(decoded) => decoded.into_owned(),
            Err(_) => return false,
        };

        for forbidden in [':', '*', '"', '<', '>', '|'] {
            if file_uri.contains(forbidden) {
                if let Some(response) = Settings::get_instance()
                    .get_default_responses()
                    .get_response(StatusCode::BAD_REQUEST, connection)
                {
                    connection.send(response);
                }

                return true;
            }
        }

        let site_directory = Settings::get_instance().get_site_directory();

        if site_directory.is_empty() {
            return false;
        }

        let path = std::path::Path::new(&site_directory).join(&file_uri);

        if !path.exists() {
            return false;
        }

        if path.is_file() {
            return Self::serve_file(connection, &path);
        }

        for index_name in ["index.htm", "index.html"] {
            let index_path = path.join(index_name);

            if index_path.is_file() {
                return Self::serve_file(connection, &index_path);
            }
        }

        if let Some(response) = Settings::get_instance()
            .get_default_responses()
            .get_response(StatusCode::NOT_FOUND, connection)
        {
            connection.send(response);
        }

        true
    }

    /// Mirrors `ResponseBuilder.create(File, WebConnection)`.
    fn serve_file(connection: &WebConnection, file: &std::path::Path) -> bool {
        let metadata = match std::fs::metadata(file) {
            Ok(metadata) => metadata,
            Err(_) => {
                if let Some(response) = Settings::get_instance()
                    .get_default_responses()
                    .get_response(StatusCode::NOT_FOUND, connection)
                {
                    connection.send(response);
                }

                return true;
            }
        };

        let file_last_modified = metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        if let Some(if_modified_since) = connection
            .request()
            .headers()
            .get("If-Modified-Since")
        {
            if !if_modified_since.is_empty() {
                if let Ok(parsed) = chrono::NaiveDateTime::parse_from_str(
                    &if_modified_since,
                    "%a, %d %b %Y %H:%M:%S GMT",
                ) {
                    if parsed.and_utc().timestamp() == file_last_modified as i64 {
                        let now = Self::now_secs();
                        let mut response =
                            ResponseBuilder::create_with_status(StatusCode::NOT_MODIFIED, "");
                        response.headers.push(("Connection".to_string(), "close".to_string()));

                        if let Some(date) = Self::http_date(now) {
                            response.headers.push(("Date".to_string(), date));
                        }

                        connection.send(response);
                        return true;
                    }
                }
            }
        }

        let data = match std::fs::read(file) {
            Ok(data) => data,
            Err(_) => {
                if let Some(response) = Settings::get_instance()
                    .get_default_responses()
                    .get_response(StatusCode::NOT_FOUND, connection)
                {
                    connection.send(response);
                }

                return true;
            }
        };

        let file_name = file
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let extension = mime_type::get_file_extension(&file_name);

        let mut response = ResponseBuilder::create_with_content_type_bytes(
            StatusCode::OK,
            mime_type::get_content_type(extension).unwrap_or("application/octet-stream"),
            data,
        );

        let now = Self::now_secs();
        let cache_renew = Settings::get_instance().get_cache_renew_time() as i64;

        if let Some(date) = Self::http_date(now) {
            response.headers.push(("Date".to_string(), date));
        }
        if let Some(expires) = Self::http_date(now + cache_renew) {
            response.headers.push(("Expires".to_string(), expires));
        }
        response
            .headers
            .push(("Cache-Control".to_string(), format!("private, max-age={cache_renew}")));
        if let Some(last_modified) = Self::http_date(file_last_modified as i64) {
            response.headers.push(("Last-Modified".to_string(), last_modified));
        }

        connection.send(response);
        true
    }

    fn http_date(secs: i64) -> Option<String> {
        chrono::DateTime::from_timestamp(secs, 0)
            .map(|date_time| date_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
    }

    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or(0)
    }
}

/// Mirrors `org.alexdev.duckhttpd.server.WebServer`.
pub struct WebServer {
    port: i32,
}

impl WebServer {
    /// Mirrors the `WebServer(int)` constructor.
    pub fn new(port: i32) -> Self {
        Self { port }
    }

    /// Mirrors `start()`.
    pub fn start(&self) {
        let port = self.port;

        tracing::info!("WebServer listening on port {port}");

        let server_thread = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("unable to build the web server runtime");

            runtime.block_on(Self::serve(port));
        });

        if server_thread.join().is_err() {
            tracing::error!("web server thread panicked");
        }
    }

    async fn serve(port: i32) {
        let app = axum::Router::new().fallback(Self::catch_all);
        let address = std::net::SocketAddr::from(([0, 0, 0, 0], port as u16));

        axum::Server::bind(&address)
            .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
            .await
            .expect("web server terminated with an error");
    }

    async fn catch_all(
        ConnectInfo(peer): ConnectInfo<std::net::SocketAddr>,
        request: http::Request<axum::body::Body>,
    ) -> impl axum::response::IntoResponse {
        let (parts, body) = request.into_parts();
        let body = body.collect().await.unwrap_or_default().to_bytes();

        let request = http::Request::from_parts(parts, Vec::from(body));
        let peer_ip = peer.ip().to_string();

        // The route handlers are synchronous and reach the DAOs, which
        // `block_on` the dedicated storage runtime; that panics when the
        // current thread is an async worker, so run the whole per-request work
        // on the blocking pool. `WebConnection` uses `RefCell` (not `Send`),
        // so it is created inside the blocking closure.
        tokio::task::spawn_blocking(move || {
            let connection = WebConnection::from_request(&request, Some(peer_ip));

            RouteManager::get_instance().invoke(&connection);

            Self::to_axum_response(&connection)
        })
        .await
        .expect("web request blocking task panicked")
    }

    fn to_axum_response(connection: &WebConnection) -> http::Response<axum::body::Body> {
        let response = connection.take_response().unwrap_or_else(|| {
            ResponseBuilder::create_with_status(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
        });

        let mut builder = axum::response::Response::builder().status(response.status);

        for (name, value) in &response.headers {
            if let (Ok(name), Ok(value)) = (
                http::HeaderName::from_bytes(name.as_bytes()),
                http::HeaderValue::from_str(value),
            ) {
                builder = builder.header(name, value);
            }
        }

        for cookie in connection.cookies().take_set_cookies() {
            if let Ok(value) = http::HeaderValue::from_str(&cookie) {
                builder = builder.header("Set-Cookie", value);
            }
        }

        builder
            .body(axum::body::Body::from(response.body))
            .unwrap_or_default()
    }
}

/// Mirrors `org.alexdev.duckhttpd.exceptions.NoServerResponseException` and the other
/// web exceptions passed to `WebResponses`.
pub enum WebException {
    NoServerResponse,
    Error(String),
}

impl std::fmt::Display for WebException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebException::NoServerResponse => write!(f, "NoServerResponseException"),
            WebException::Error(message) => write!(f, "{message}"),
        }
    }
}

/// Mirrors `org.alexdev.duckhttpd.response.Response`.
#[derive(Clone, Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: Vec<u8>,
    pub headers: Vec<(String, String)>,
}

impl Response {
    /// Mirrors `headers()` (view over the pending response headers).
    pub fn headers(&mut self) -> &mut Vec<(String, String)> {
        &mut self.headers
    }

    /// Mirrors `headers().set(String, String)`.
    pub fn set_header(&mut self, name: &str, value: &str) {
        if let Some(entry) = self
            .headers
            .iter_mut()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
        {
            entry.1 = value.to_string();
        } else {
            self.headers.push((name.to_string(), value.to_string()));
        }
    }
}

/// Mirrors `org.alexdev.duckhttpd.response.ResponseBuilder`.
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// Mirrors `ResponseBuilder.create(String)`.
    pub fn create(body: impl Into<String>) -> Response {
        Self::create_with_status(StatusCode::OK, body)
    }

    /// Mirrors `ResponseBuilder.create(HttpResponseStatus, String)`.
    pub fn create_with_status(status: StatusCode, body: impl Into<String>) -> Response {
        let mut response = Response {
            status,
            body: body.into().into_bytes(),
            headers: Vec::new(),
        };

        Self::apply_content_type(&mut response, "text/html");
        Self::apply_default_headers(&mut response);
        response
    }

    /// Mirrors `ResponseBuilder.create(String, String)` (content type + body).
    pub fn create_with_content_type(content_type: &str, body: impl Into<String>) -> Response {
        let mut response = Response {
            status: StatusCode::OK,
            body: body.into().into_bytes(),
            headers: Vec::new(),
        };

        Self::apply_content_type(&mut response, content_type);
        Self::apply_default_headers(&mut response);
        response
    }

    /// Mirrors `ResponseBuilder.create(HttpResponseStatus, String, byte[])`.
    pub fn create_with_content_type_bytes(
        status: StatusCode,
        content_type: &str,
        body: Vec<u8>,
    ) -> Response {
        let mut response = Response {
            status,
            body,
            headers: Vec::new(),
        };

        Self::apply_content_type(&mut response, content_type);
        Self::apply_default_headers(&mut response);
        response
    }

    fn apply_content_type(response: &mut Response, content_type: &str) {
        let mut value = content_type.to_string();

        if let Some(encoding) = Settings::get_instance().get_page_encoding() {
            value.push_str("; charset=");
            value.push_str(&encoding);
        }

        response
            .headers
            .push(("Content-Type".to_string(), value));
    }

    fn apply_default_headers(response: &mut Response) {
        for (name, value) in Settings::get_instance().get_default_headers() {
            response.headers.push((name, value));
        }
    }
}

/// Mirrors a value stored in a template context (Java `HashMap<String, Object>`).
/// Values are materialised as JSON (tera has no object-property access like Pebble).
#[derive(Clone, Debug)]
pub struct TemplateValue(serde_json::Value);

impl TemplateValue {
    pub fn json(value: serde_json::Value) -> Self {
        Self(value)
    }

    pub fn of<T: Serialize>(value: T) -> Self {
        Self(serde_json::to_value(value).unwrap_or(serde_json::Value::Null))
    }

    /// Returns the underlying JSON value (so a value can be materialised
    /// back into its original type, mirroring the Java object cast of
    /// `Template.get(String)`).
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl Serialize for TemplateValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// Mirrors the `org.alexdev.duckhttpd.response.WebResponses` interface.
pub trait WebResponses {
    /// Mirrors `getErrorResponse(WebConnection, Throwable)`.
    fn get_error_response(
        &self,
        client: &WebConnection,
        throwable: Option<&WebException>,
    ) -> Option<Response>;

    /// Mirrors `getResponse(HttpResponseStatus, WebConnection)`.
    fn get_response(&self, status: StatusCode, web_connection: &WebConnection) -> Option<Response>;
}

/// Mirrors `org.alexdev.duckhttpd.util.config.Settings`.
pub struct Settings;

static TEMPLATE_BASE_SET: AtomicBool = AtomicBool::new(false);
static SITE_DIRECTORY: OnceLock<String> = OnceLock::new();
static PAGE_ENCODING: OnceLock<String> = OnceLock::new();
static CACHE_RENEW_TIME: OnceLock<i32> = OnceLock::new();
static SAVE_SESSIONS: AtomicBool = AtomicBool::new(true);
lazy_static::lazy_static! {
    static ref BLOCK_IPV4: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref DEFAULT_HEADERS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    static ref DEFAULT_RESPONSES: Mutex<Option<Arc<dyn WebResponses + Send + Sync>>> =
        Mutex::new(None);
}

impl Settings {
    /// Mirrors `Settings.getInstance()`.
    pub fn get_instance() -> &'static Settings {
        static INSTANCE: Settings = Settings;
        &INSTANCE
    }

    /// Mirrors `getDefaultResponses()`.
    pub fn get_default_responses(&self) -> Arc<dyn WebResponses + Send + Sync> {
        DEFAULT_RESPONSES
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| Arc::new(crate::server::server_responses::ServerResponses))
    }

    /// Mirrors `setSiteDirectory(String)`.
    pub fn set_site_directory(&self, path: &str) {
        let _ = SITE_DIRECTORY.set(path.to_string());
    }

    /// Mirrors `getSiteDirectory()`.
    pub fn get_site_directory(&self) -> String {
        SITE_DIRECTORY.get().cloned().unwrap_or_default()
    }

    /// Mirrors `setDefaultResponses(WebResponses)`.
    pub fn set_default_responses(&self, responses: Box<dyn WebResponses + Send + Sync>) {
        *DEFAULT_RESPONSES.lock().unwrap() = Some(Arc::from(responses));
    }

    /// Mirrors `setTemplateBase(Class<? extends Template>)`.
    ///
    /// The port has a single concrete template type (`TwigTemplate`), so
    /// setting the base is recorded as a flag (the base is always
    /// `TwigTemplate`).
    pub fn set_template_base(&self, _template_base: &TwigTemplate) {
        TEMPLATE_BASE_SET.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Mirrors `setSaveSessions(boolean)`.
    pub fn set_save_sessions(&self, save_sessions: bool) {
        SAVE_SESSIONS.store(save_sessions, std::sync::atomic::Ordering::SeqCst);
    }

    /// Mirrors `isSaveSessions()`.
    pub fn is_save_sessions(&self) -> bool {
        SAVE_SESSIONS.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Mirrors `setPageEncoding(String)`.
    pub fn set_page_encoding(&self, encoding: &str) {
        let _ = PAGE_ENCODING.set(encoding.to_string());
    }

    /// Mirrors `getPageEncoding()`.
    pub fn get_page_encoding(&self) -> Option<String> {
        PAGE_ENCODING.get().cloned()
    }

    /// Mirrors `getCacheRenewTime()`.
    pub fn get_cache_renew_time(&self) -> i32 {
        CACHE_RENEW_TIME.get_or_init(|| (7 * 24 * 60 * 60) as i32).clone()
    }

    /// Mirrors `setCacheRenewTime(int)`.
    pub fn set_cache_renew_time(&self, cache_renew_time: i32) {
        let _ = CACHE_RENEW_TIME.set(cache_renew_time);
    }

    /// Mirrors `getBlockIpv4()`.
    pub fn get_block_ipv4(&self) -> Vec<String> {
        BLOCK_IPV4.lock().unwrap().clone()
    }

    /// Mirrors `getDefaultHeaders()`.
    pub fn get_default_headers(&self) -> Vec<(String, String)> {
        DEFAULT_HEADERS.lock().unwrap().clone()
    }
}

/// Mirrors the `org.alexdev.duckhttpd.template.TemplateBinder` interface.
pub trait TemplateBinder {
    fn on_register(
        &mut self,
        template: &mut dyn Template,
        web_connection: Option<&WebConnection>,
    );
}

/// Mirrors the `org.alexdev.duckhttpd.template.Template` base class.
pub trait Template {
    fn start(&mut self, view: &str);
    fn set(&mut self, name: &str, value: TemplateValue);
    fn get(&self, name: &str) -> Option<&TemplateValue>;
    fn render(&mut self);
    fn register_binder(&mut self, binder: Box<dyn TemplateBinder>);
    fn web_connection(&self) -> Option<&WebConnection>;
}
