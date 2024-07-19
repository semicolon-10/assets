use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate{}

pub struct HtmlTemplate<T>(pub T);

impl <T> IntoResponse for HtmlTemplate<T>
where T: Template, {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(ex) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", ex)
            ).into_response()
        }
    }
}
