//! Mirrors `org.alexdev.http.template.binders.SessionBinder`.

use crate::duckhttpd::{Template, TemplateBinder, TemplateValue, WebConnection};

/// Mirrors `org.alexdev.http.template.binders.SessionBinder`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SessionBinder {
    #[serde(rename = "loggedIn")]
    logged_in: bool,
    #[serde(rename = "currentPage", skip_serializing_if = "Option::is_none")]
    current_page: Option<String>,
}

impl SessionBinder {
    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            logged_in: false,
            current_page: None,
        }
    }

    /// Mirrors `isLoggedIn()`.
    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    /// Mirrors `getCurrentPage()`.
    pub fn get_current_page(&self) -> Option<&str> {
        self.current_page.as_deref()
    }
}

impl TemplateBinder for SessionBinder {
    /// Mirrors `onRegister(Template, WebConnection)`.
    fn on_register(
        &mut self,
        template: &mut dyn Template,
        web_connection: Option<&WebConnection>,
    ) {
        if let Some(connection) = web_connection {
            self.current_page = connection.session().get_string("page");

            if connection.session().get_boolean("authenticated") {
                self.logged_in = true;
            }
        }

        template.set("session", TemplateValue::of(self.clone()));
    }
}
