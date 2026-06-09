use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub fn render<T: Template>(template: T) -> impl IntoResponse {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(error) => {
            eprintln!("Template error: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Template rendering error",
            )
                .into_response()
        }
    }
}
