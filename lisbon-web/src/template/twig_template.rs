//! Mirrors `org.alexdev.http.template.TwigTemplate`.

use std::collections::HashMap;
use std::path::Path;

use tera::Tera;

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::util::config::server_configuration::ServerConfiguration;

use crate::duckhttpd::{
    Settings, Template, TemplateBinder, TemplateValue, WebConnection, WebException,
};
use crate::log::Log;
use crate::template::binders::alert_binder::AlertBinder;
use crate::template::binders::session_binder::SessionBinder;
use crate::template::binders::site_binder::SiteBinder;

/// Mirrors `org.alexdev.http.template.TwigTemplate` (Pebble engine, misnamed in Java).
pub struct TwigTemplate<'a> {
    file: Option<std::path::PathBuf>,
    view: Option<String>,
    engine: Option<Tera>,
    context: HashMap<String, TemplateValue>,
    binders: Vec<Box<dyn TemplateBinder>>,
    web_connection: Option<&'a WebConnection>,
}

impl<'a> TwigTemplate<'a> {
    /// Mirrors the constructor `TwigTemplate(WebConnection)`.
    pub fn new(web_connection: Option<&'a WebConnection>) -> Self {
        Self {
            file: None,
            view: None,
            engine: None,
            context: HashMap::new(),
            binders: Vec::new(),
            web_connection,
        }
    }

    /// Mirrors `start(String)`.
    pub fn start(&mut self, view: &str) {
        self.view = Some(view.to_string());

        let path = Path::new(&ServerConfiguration::get_string("template.directory"))
            .join(ServerConfiguration::get_string("template.name"))
            .join(format!("{view}.tpl"));

        if path.exists() && path.is_file() {
            self.file = Some(path.clone());

            let mut engine = Tera::default();
            engine.autoescape_on(vec![]);
            match engine.add_raw_template(
                format!("{view}.tpl").as_str(),
                std::fs::read_to_string(&path).unwrap_or_default().as_str(),
            ) {
                Ok(_) => self.engine = Some(engine),
                Err(ex) => {
                    self.report_error(&ex.to_string());
                    return;
                }
            }
        } else {
            self.report_error(&format!(
                "The template view {view} does not exist!\nThe path: {}",
                path.display()
            ));
            return;
        }

        if let Some(user_id) = self.authenticated_user_id() {
            match PlayerDao::get_details(user_id) {
                None => {
                    if let Some(connection) = self.web_connection {
                        connection.session().delete("authenticated");
                        connection.redirect("/");
                    }
                    return;
                }
                Some(details) => {
                    self.set("playerDetails", TemplateValue::of(details));
                }
            }
        }
    }

    fn authenticated_user_id(&self) -> Option<i32> {
        let connection = self.web_connection?;
        let session = connection.session();
        if session.get_boolean("authenticated")
            || session.get_boolean("authenticatedHousekeeping")
        {
            Some(session.get_int("user.id"))
        } else {
            None
        }
    }

    fn report_error(&self, error: &str) {
        if let Some(connection) = self.web_connection {
            let _ = Settings::get_instance()
                .get_default_responses()
                .get_error_response(connection, Some(&WebException::Error(error.to_string())));
        } else {
            Log::get_error_logger().error_with("Error: ", error);
        }
    }

    /// Mirrors `set(String, Object)`.
    pub fn set(&mut self, name: &str, value: TemplateValue) {
        self.context.insert(name.to_string(), value);
    }

    /// Mirrors `get(String)`.
    pub fn get(&self, name: &str) -> Option<&TemplateValue> {
        self.context.get(name)
    }

    /// Mirrors `attachBinders()`.
    fn attach_binders(&mut self) {
        self.register_binder(Box::new(SessionBinder::new()));
        self.register_binder(Box::new(SiteBinder::new()));

        if let Some((message, colour)) = self.alert_values() {
            self.register_binder(Box::new(AlertBinder::new(message, colour)));
        }
    }

    fn alert_values(&self) -> Option<(Option<String>, Option<String>)> {
        let connection = self.web_connection?;
        Some((
            connection.session().get_string("alertMessage"),
            connection.session().get_string("alertColour"),
        ))
    }

    /// Mirrors `renderHTML()`.
    pub fn render_html(&mut self) -> Result<String, String> {
        self.attach_binders();

        match self.engine.as_ref() {
            Some(engine) => {
                let view_name = self.view.as_deref().unwrap_or_default();

                let mut context = tera::Context::new();
                for (key, value) in &self.context {
                    context.insert(key, value);
                }

                engine.render(view_name, &context).map_err(|ex| ex.to_string())
            }
            None => Err("template engine not initialised".to_string()),
        }
    }

    /// Mirrors `render()`.
    pub fn render(&mut self) {
        match self.render_html() {
            Ok(html) => {
                if let Some(connection) = self.web_connection {
                    let mut response = crate::duckhttpd::ResponseBuilder::create(html);
                    for (key, value) in connection.headers() {
                        response.headers().push((key, value));
                    }
                    connection.send(response);
                    connection.clear_headers();
                }
            }
            Err(ex) => {
                if let Some(connection) = self.web_connection {
                    let _ = Settings::get_instance()
                        .get_default_responses()
                        .get_error_response(connection, Some(&WebException::Error(ex)));
                } else {
                    Log::get_error_logger().error(&ex);
                }
            }
        }
    }
}

impl<'a> Template for TwigTemplate<'a> {
    fn start(&mut self, view: &str) {
        self.start(view);
    }

    fn set(&mut self, name: &str, value: TemplateValue) {
        self.set(name, value);
    }

    fn get(&self, name: &str) -> Option<&TemplateValue> {
        self.get(name)
    }

    fn render(&mut self) {
        self.render();
    }

    fn register_binder(&mut self, mut binder: Box<dyn TemplateBinder>) {
        let web_connection = self.web_connection;
        binder.on_register(self, web_connection);
        self.binders.push(binder);
    }

    fn web_connection(&self) -> Option<&WebConnection> {
        self.web_connection
    }
}
