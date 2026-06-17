use axum::{extract::Query, response::IntoResponse};
use std::collections::HashMap;

use super::{
    render::render,
    service_content::services_overview_context,
    templates::{
        AboutTemplate, ContactTemplate, FaqTemplate, RequestQuoteTemplate, ServicesTemplate,
    },
};

pub async fn about() -> impl IntoResponse {
    render(AboutTemplate)
}

pub async fn services() -> impl IntoResponse {
    let context = services_overview_context();

    render(ServicesTemplate {
        services: context.services,
        proof_points: context.proof_points,
        process: context.process,
    })
}

pub async fn contact(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let success = params.get("success").is_some_and(|value| value == "1");

    render(ContactTemplate { success })
}

pub async fn faq() -> impl IntoResponse {
    render(FaqTemplate)
}

pub async fn request_quote() -> impl IntoResponse {
    render(RequestQuoteTemplate)
}
