use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use std::collections::HashMap;

use super::{
    package_content::{
        app_software_family, find_app_software_package, find_web_package, web_design_family,
    },
    render::render,
    service_content::services_overview_context,
    templates::{
        AboutTemplate, BusinessSeoOfferTemplate, BusinessWebsitePackageTemplate, ContactTemplate,
        FaqTemplate, FounderPortfolioTemplate, NotFoundTemplate, PackagesTemplate,
        RequestQuoteTemplate, ServicesTemplate, WebDesignPackageDetailTemplate,
        WebDesignPackagesTemplate,
    },
};

pub async fn about() -> impl IntoResponse {
    render(AboutTemplate)
}

pub async fn founder_portfolio() -> impl IntoResponse {
    render(FounderPortfolioTemplate)
}

pub async fn packages() -> impl IntoResponse {
    render(PackagesTemplate)
}

pub async fn business_website_package() -> impl IntoResponse {
    render(BusinessWebsitePackageTemplate)
}

pub async fn business_seo_offer() -> impl IntoResponse {
    render(BusinessSeoOfferTemplate)
}

pub async fn web_design_development_packages() -> impl IntoResponse {
    render(WebDesignPackagesTemplate {
        family: web_design_family(),
    })
}

pub async fn web_design_development_package(Path(slug): Path<String>) -> impl IntoResponse {
    match find_web_package(&slug) {
        Some(package) => render(WebDesignPackageDetailTemplate { package }).into_response(),
        None => not_found().await.into_response(),
    }
}

pub async fn app_software_development_packages() -> impl IntoResponse {
    render(WebDesignPackagesTemplate {
        family: app_software_family(),
    })
}

pub async fn app_software_development_package(Path(slug): Path<String>) -> impl IntoResponse {
    match find_app_software_package(&slug) {
        Some(package) => render(WebDesignPackageDetailTemplate { package }).into_response(),
        None => not_found().await.into_response(),
    }
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

pub async fn portfolio_alias() -> impl IntoResponse {
    Redirect::permanent("/case-studies")
}

pub async fn portfolio_single_alias(Path(slug): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/case-studies/{slug}"))
}

pub async fn service_area_alias() -> impl IntoResponse {
    Redirect::permanent("/service-areas")
}

pub async fn service_area_single_alias(Path(slug): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/service-areas/{slug}"))
}

pub async fn robot_alias() -> impl IntoResponse {
    Redirect::permanent("/robots.txt")
}

pub async fn llmo_alias() -> impl IntoResponse {
    Redirect::permanent("/llms.txt")
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, render(NotFoundTemplate))
}
