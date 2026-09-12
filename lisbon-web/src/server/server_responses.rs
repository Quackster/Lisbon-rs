//! Mirrors `org.alexdev.http.server.ServerResponses`.

use http::StatusCode;

use crate::duckhttpd::{
    ResponseBuilder, WebConnection, WebException, WebResponses,
};
use crate::log::Log;
use crate::template::twig_template::TwigTemplate;

/// Mirrors `org.alexdev.http.server.ServerResponses`.
pub struct ServerResponses;

impl WebResponses for ServerResponses {
    /// Mirrors `getErrorResponse(WebConnection, Throwable)`.
    fn get_error_response(
        &self,
        client: &WebConnection,
        throwable: Option<&WebException>,
    ) -> Option<crate::duckhttpd::Response> {
        if let Some(exception) = throwable {
            if matches!(exception, WebException::NoServerResponse) {
                Log::get_error_logger()
                    .error(format!("Server did not send response for: {}", client.get_route_request()));
            }
            Log::get_error_logger().error_with("Error occurred: ", exception);
        }

        client.session().delete("page");

        let mut tpl = client.template("overrides/default");
        match tpl.render_html() {
            Ok(html) => Some(ResponseBuilder::create(html)),
            Err(error) => {
                Log::get_error_logger().error(&error);
                None
            }
        }
    }

    /// Mirrors `getResponse(HttpResponseStatus, WebConnection)`.
    fn get_response(
        &self,
        status: StatusCode,
        web_connection: &WebConnection,
    ) -> Option<crate::duckhttpd::Response> {
        web_connection.session().delete("page");

        if status == StatusCode::FORBIDDEN {
            return Some(ResponseBuilder::create_with_status(
                StatusCode::FORBIDDEN,
                "\n<html>\n<head>\n</head>\n<body>\n   <h1>Forbidden</h1>\n<body>\n</html>",
            ));
        }

        if status == StatusCode::NOT_FOUND {
            let mut twig_template = TwigTemplate::new(Some(web_connection));
            twig_template.start("overrides/default");
            match twig_template.render_html() {
                Ok(html) => {
                    return Some(ResponseBuilder::create_with_status(
                        StatusCode::NOT_FOUND,
                        html,
                    ))
                }
                Err(error) => Log::get_error_logger().error(&error),
            }
        }

        Some(ResponseBuilder::create_with_status(
            StatusCode::BAD_REQUEST,
            "\n<html>\n<head>\n</head>\n<body>\n   <h1>Bad Request</h1>\n<body>\n</html>",
        ))
    }
}
