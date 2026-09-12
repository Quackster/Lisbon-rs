//! Mirrors `org.alexdev.http.template.binders.AlertBinder`.

use crate::duckhttpd::{Template, TemplateBinder, TemplateValue, WebConnection};

/// Mirrors `org.alexdev.http.template.binders.AlertBinder`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AlertBinder {
    #[serde(rename = "hasAlert")]
    has_alert: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
}

impl AlertBinder {
    /// Mirrors `AlertBinder(String, String)`.
    pub fn new(message: Option<String>, colour: Option<String>) -> Self {
        if message.is_some() {
            Self {
                has_alert: true,
                message,
                colour,
            }
        } else {
            Self {
                has_alert: false,
                message: None,
                colour: None,
            }
        }
    }

    /// Mirrors `isHasAlert()`.
    pub fn is_has_alert(&self) -> bool {
        self.has_alert
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Mirrors `getColour()`.
    pub fn get_colour(&self) -> Option<&str> {
        self.colour.as_deref()
    }
}

impl TemplateBinder for AlertBinder {
    /// Mirrors `onRegister(Template, WebConnection)`.
    fn on_register(
        &mut self,
        template: &mut dyn Template,
        _web_connection: Option<&WebConnection>,
    ) {
        template.set("alert", TemplateValue::of(self.clone()));
    }
}
