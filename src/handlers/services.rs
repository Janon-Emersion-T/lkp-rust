use axum::response::{IntoResponse, Redirect};

use super::{
    render::render,
    service_content::service_page_context,
    templates::{
        AiAutomationTemplate, BookingSystemDevelopmentTemplate, CrmSoftwareDevelopmentTemplate,
        CustomSoftwareDevelopmentTemplate, DigitalMarketingTemplate, GoogleAdsAgencyTemplate,
        HostingDomainCloudTemplate, InventoryManagementSoftwareTemplate, ItConsultationTemplate,
        LocalSeoServicesTemplate, MobileAppDevelopmentTemplate, SeoSearchGrowthTemplate,
        SoftwareDevelopmentTemplate, WebDevelopmentTemplate, WebsiteMaintenanceTemplate,
        WebsiteRedesignTemplate,
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

pub async fn google_ads_agency() -> impl IntoResponse {
    render(GoogleAdsAgencyTemplate {
        page: service_page_context("google-ads-agency"),
    })
}

pub async fn local_seo_services() -> impl IntoResponse {
    render(LocalSeoServicesTemplate {
        page: service_page_context("local-seo-services"),
    })
}

pub async fn hosting_domain_cloud() -> impl IntoResponse {
    render(HostingDomainCloudTemplate {
        page: service_page_context("hosting-domain-cloud-services"),
    })
}

pub async fn website_maintenance() -> impl IntoResponse {
    render(WebsiteMaintenanceTemplate {
        page: service_page_context("website-maintenance-services"),
    })
}

pub async fn website_redesign() -> impl IntoResponse {
    render(WebsiteRedesignTemplate {
        page: service_page_context("website-redesign-services"),
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

pub async fn crm_software_development() -> impl IntoResponse {
    render(CrmSoftwareDevelopmentTemplate {
        page: service_page_context("crm-software-development"),
    })
}

pub async fn inventory_management_software() -> impl IntoResponse {
    render(InventoryManagementSoftwareTemplate {
        page: service_page_context("inventory-management-software"),
    })
}

pub async fn booking_system_development() -> impl IntoResponse {
    render(BookingSystemDevelopmentTemplate {
        page: service_page_context("booking-system-development"),
    })
}

pub async fn web_development_alias() -> impl IntoResponse {
    Redirect::permanent("/services/web-development")
}

pub async fn mobile_app_development_alias() -> impl IntoResponse {
    Redirect::permanent("/services/mobile-app-development")
}

pub async fn custom_software_development_alias() -> impl IntoResponse {
    Redirect::permanent("/services/custom-software-development")
}

pub async fn software_development_alias() -> impl IntoResponse {
    Redirect::permanent("/services/software-development")
}

pub async fn digital_marketing_alias() -> impl IntoResponse {
    Redirect::permanent("/services/digital-marketing")
}

pub async fn seo_alias() -> impl IntoResponse {
    Redirect::permanent("/services/seo-search-growth")
}

pub async fn hosting_domain_cloud_alias() -> impl IntoResponse {
    Redirect::permanent("/services/hosting-domain-cloud-services")
}

pub async fn hosting_support_alias() -> impl IntoResponse {
    Redirect::permanent("/services/website-maintenance-services")
}

pub async fn ai_automation_alias() -> impl IntoResponse {
    Redirect::permanent("/services/ai-automation-solutions")
}

pub async fn it_consultation_alias() -> impl IntoResponse {
    Redirect::permanent("/services/it-consultation-digital-transformation")
}
