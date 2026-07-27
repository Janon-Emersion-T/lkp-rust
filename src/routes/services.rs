use axum::{Router, routing::get};

use crate::{
    handlers::{
        ai_automation, ai_automation_alias, booking_system_development, crm_software_development,
        custom_software_development, custom_software_development_alias, digital_marketing,
        digital_marketing_alias, google_ads_agency, hosting_domain_cloud,
        hosting_domain_cloud_alias, hosting_support_alias, inventory_management_software,
        it_consultation, it_consultation_alias, local_seo_services, mobile_app_development,
        mobile_app_development_alias, seo_alias, seo_search_growth, software_development,
        software_development_alias, web_development, web_development_alias, website_maintenance,
        website_redesign,
    },
    state::AppState,
};

pub fn service_routes() -> Router<AppState> {
    Router::new()
        .route("/services/web-development", get(web_development))
        .route("/services/webdevelopment", get(web_development_alias))
        .route(
            "/services/mobile-app-development",
            get(mobile_app_development),
        )
        .route(
            "/services/mobileappdevelopment",
            get(mobile_app_development_alias),
        )
        .route(
            "/services/custom-software-development",
            get(custom_software_development),
        )
        .route(
            "/services/customsoftwaredevelopment",
            get(custom_software_development_alias),
        )
        .route("/services/software-development", get(software_development))
        .route(
            "/services/softwaredevelopment",
            get(software_development_alias),
        )
        .route("/services/digital-marketing", get(digital_marketing))
        .route("/services/digitalmarketing", get(digital_marketing_alias))
        .route("/services/google-ads-agency", get(google_ads_agency))
        .route("/services/seo-search-growth", get(seo_search_growth))
        .route("/services/local-seo-services", get(local_seo_services))
        .route("/services/seo", get(seo_alias))
        .route(
            "/services/hosting-domain-cloud-services",
            get(hosting_domain_cloud),
        )
        .route(
            "/services/hosting-domain-cloud",
            get(hosting_domain_cloud_alias),
        )
        .route("/services/hosting", get(hosting_domain_cloud_alias))
        .route(
            "/services/website-maintenance-services",
            get(website_maintenance),
        )
        .route("/services/website-support", get(hosting_support_alias))
        .route("/services/website-redesign-services", get(website_redesign))
        .route("/services/ai-automation-solutions", get(ai_automation))
        .route("/services/ai-automation", get(ai_automation_alias))
        .route(
            "/services/crm-software-development",
            get(crm_software_development),
        )
        .route(
            "/services/inventory-management-software",
            get(inventory_management_software),
        )
        .route(
            "/services/booking-system-development",
            get(booking_system_development),
        )
        .route(
            "/services/it-consultation-digital-transformation",
            get(it_consultation),
        )
        .route("/services/it-consultation", get(it_consultation_alias))
}
