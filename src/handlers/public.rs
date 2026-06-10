use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use std::collections::HashMap;

use super::{
    render::render,
    templates::{
        AboutTemplate, CareerApplyTemplate, CareerSingleTemplate, CareersTemplate, ContactTemplate,
        HomeTemplate, IndustriesTemplate, InsightSingleTemplate, InsightsTemplate,
        PortfolioSingleTemplate, PortfolioTemplate, RequestQuoteTemplate, ServicesTemplate,
    },
};

pub async fn home() -> impl IntoResponse {
    render(HomeTemplate)
}

pub async fn about() -> impl IntoResponse {
    render(AboutTemplate)
}

pub async fn services() -> impl IntoResponse {
    render(ServicesTemplate)
}

pub async fn industries() -> impl IntoResponse {
    render(IndustriesTemplate)
}

pub async fn portfolio() -> impl IntoResponse {
    render(PortfolioTemplate)
}

pub async fn insights() -> impl IntoResponse {
    render(InsightsTemplate)
}

pub async fn careers() -> impl IntoResponse {
    render(CareersTemplate)
}

pub async fn contact(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let success = params.get("success").is_some_and(|value| value == "1");

    render(ContactTemplate { success })
}

pub async fn request_quote() -> impl IntoResponse {
    render(RequestQuoteTemplate)
}

pub async fn portfolio_single(Path(slug): Path<String>) -> impl IntoResponse {
    render(PortfolioSingleTemplate { slug })
}

pub async fn insight_single(Path(slug): Path<String>) -> impl IntoResponse {
    render(InsightSingleTemplate { slug })
}

pub async fn career_single(Path(slug): Path<String>) -> impl IntoResponse {
    render(CareerSingleTemplate { slug })
}

pub async fn career_apply(Path(slug): Path<String>) -> impl IntoResponse {
    render(CareerApplyTemplate { slug })
}
