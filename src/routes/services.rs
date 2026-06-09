use axum::{Router, routing::get};

use crate::{
    handlers::{
        ai_automation, custom_software_development, digital_marketing, hosting_domain_cloud,
        it_consultation, mobile_app_development, seo_search_growth, software_development,
        web_development,
    },
    state::AppState,
};

pub fn service_routes() -> Router<AppState> {
    Router::new()
        .route("/services/web-development", get(web_development))
        .route(
            "/services/mobile-app-development",
            get(mobile_app_development),
        )
        .route(
            "/services/custom-software-development",
            get(custom_software_development),
        )
        .route("/services/software-development", get(software_development))
        .route("/services/digital-marketing", get(digital_marketing))
        .route("/services/seo-search-growth", get(seo_search_growth))
        .route(
            "/services/hosting-domain-cloud-services",
            get(hosting_domain_cloud),
        )
        .route("/services/ai-automation-solutions", get(ai_automation))
        .route(
            "/services/it-consultation-digital-transformation",
            get(it_consultation),
        )
}
