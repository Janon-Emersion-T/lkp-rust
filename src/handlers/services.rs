use axum::response::IntoResponse;

use super::{
    render::render,
    templates::{
        AiAutomationTemplate, CustomSoftwareDevelopmentTemplate, DigitalMarketingTemplate,
        HostingDomainCloudTemplate, ItConsultationTemplate, MobileAppDevelopmentTemplate,
        SeoSearchGrowthTemplate, SoftwareDevelopmentTemplate, WebDevelopmentTemplate,
    },
};

pub async fn web_development() -> impl IntoResponse {
    render(WebDevelopmentTemplate)
}

pub async fn mobile_app_development() -> impl IntoResponse {
    render(MobileAppDevelopmentTemplate)
}

pub async fn custom_software_development() -> impl IntoResponse {
    render(CustomSoftwareDevelopmentTemplate)
}

pub async fn software_development() -> impl IntoResponse {
    render(SoftwareDevelopmentTemplate)
}

pub async fn digital_marketing() -> impl IntoResponse {
    render(DigitalMarketingTemplate)
}

pub async fn seo_search_growth() -> impl IntoResponse {
    render(SeoSearchGrowthTemplate)
}

pub async fn hosting_domain_cloud() -> impl IntoResponse {
    render(HostingDomainCloudTemplate)
}

pub async fn ai_automation() -> impl IntoResponse {
    render(AiAutomationTemplate)
}

pub async fn it_consultation() -> impl IntoResponse {
    render(ItConsultationTemplate)
}
