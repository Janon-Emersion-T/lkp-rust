use axum::response::IntoResponse;

use super::{
    render::render,
    service_content::service_page_context,
    templates::{
        AiAutomationTemplate, CustomSoftwareDevelopmentTemplate, DigitalMarketingTemplate,
        HostingDomainCloudTemplate, ItConsultationTemplate, MobileAppDevelopmentTemplate,
        SeoSearchGrowthTemplate, SoftwareDevelopmentTemplate, WebDevelopmentTemplate,
    },
};

pub async fn web_development() -> impl IntoResponse {
    render(WebDevelopmentTemplate {
        page: service_page_context("web-development"),
    })
}

pub async fn mobile_app_development() -> impl IntoResponse {
    render(MobileAppDevelopmentTemplate {
        page: service_page_context("mobile-app-development"),
    })
}

pub async fn custom_software_development() -> impl IntoResponse {
    render(CustomSoftwareDevelopmentTemplate {
        page: service_page_context("custom-software-development"),
    })
}

pub async fn software_development() -> impl IntoResponse {
    render(SoftwareDevelopmentTemplate {
        page: service_page_context("software-development"),
    })
}

pub async fn digital_marketing() -> impl IntoResponse {
    render(DigitalMarketingTemplate {
        page: service_page_context("digital-marketing"),
    })
}

pub async fn seo_search_growth() -> impl IntoResponse {
    render(SeoSearchGrowthTemplate {
        page: service_page_context("seo-search-growth"),
    })
}

pub async fn hosting_domain_cloud() -> impl IntoResponse {
    render(HostingDomainCloudTemplate {
        page: service_page_context("hosting-domain-cloud-services"),
    })
}

pub async fn ai_automation() -> impl IntoResponse {
    render(AiAutomationTemplate {
        page: service_page_context("ai-automation-solutions"),
    })
}

pub async fn it_consultation() -> impl IntoResponse {
    render(ItConsultationTemplate {
        page: service_page_context("it-consultation-digital-transformation"),
    })
}
