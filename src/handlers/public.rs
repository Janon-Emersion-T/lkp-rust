use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use std::collections::HashMap;

use super::{
    render::render,
    service_area_content::{
        all_service_area_cards, related_service_areas, service_area_count,
        service_area_featured_services, service_area_groups, service_area_page,
    },
    service_content::services_overview_context,
    templates::{
        AboutTemplate, ContactTemplate, FaqTemplate, RequestQuoteTemplate,
        ServiceAreaDetailTemplate, ServiceAreasTemplate, ServicesTemplate,
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

pub async fn service_areas() -> impl IntoResponse {
    render(ServiceAreasTemplate {
        groups: service_area_groups(),
        all_areas: all_service_area_cards(),
        total_areas: service_area_count(),
        featured_services: service_area_featured_services(),
    })
}

pub async fn service_area_single(Path(slug): Path<String>) -> impl IntoResponse {
    match service_area_page(&slug) {
        Some(page) => render(ServiceAreaDetailTemplate {
            page,
            related_areas: related_service_areas(&slug),
            featured_services: service_area_featured_services(),
        })
        .into_response(),
        None => (StatusCode::NOT_FOUND, "Service area not found.").into_response(),
    }
}
