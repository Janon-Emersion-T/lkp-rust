use axum::response::IntoResponse;

use super::{
    render::render,
    templates::{
        DashboardAuditLogsTemplate, DashboardFaqCreateTemplate, DashboardFaqEditTemplate,
        DashboardFaqsTemplate, DashboardInsightCategoriesTemplate,
        DashboardInsightCategoryCreateTemplate, DashboardInsightCategoryEditTemplate,
        DashboardInsightTagCreateTemplate, DashboardInsightTagEditTemplate,
        DashboardInsightTagsTemplate, DashboardMediaTemplate, DashboardMenusTemplate,
        DashboardMilestoneCreateTemplate, DashboardMilestoneEditTemplate,
        DashboardMilestonesTemplate, DashboardPagesTemplate, DashboardProductCreateTemplate,
        DashboardProductEditTemplate, DashboardProductsTemplate, DashboardQuoteRequestsTemplate,
        DashboardSeoTemplate, DashboardServiceCreateTemplate, DashboardServiceEditTemplate,
        DashboardServicesTemplate, DashboardSettingsTemplate, DashboardTemplate,
        DashboardTestimonialCreateTemplate, DashboardTestimonialEditTemplate,
        DashboardTestimonialsTemplate, DashboardUsersTemplate,
    },
};

pub async fn dashboard() -> impl IntoResponse {
    render(DashboardTemplate)
}

pub async fn dashboard_insight_categories() -> impl IntoResponse {
    render(DashboardInsightCategoriesTemplate)
}

pub async fn dashboard_insight_category_create() -> impl IntoResponse {
    render(DashboardInsightCategoryCreateTemplate)
}

pub async fn dashboard_insight_category_edit() -> impl IntoResponse {
    render(DashboardInsightCategoryEditTemplate)
}

pub async fn dashboard_insight_tags() -> impl IntoResponse {
    render(DashboardInsightTagsTemplate)
}

pub async fn dashboard_insight_tag_create() -> impl IntoResponse {
    render(DashboardInsightTagCreateTemplate)
}

pub async fn dashboard_insight_tag_edit() -> impl IntoResponse {
    render(DashboardInsightTagEditTemplate)
}

pub async fn dashboard_milestones() -> impl IntoResponse {
    render(DashboardMilestonesTemplate)
}

pub async fn dashboard_milestone_create() -> impl IntoResponse {
    render(DashboardMilestoneCreateTemplate)
}

pub async fn dashboard_milestone_edit() -> impl IntoResponse {
    render(DashboardMilestoneEditTemplate)
}

pub async fn dashboard_products() -> impl IntoResponse {
    render(DashboardProductsTemplate)
}

pub async fn dashboard_product_create() -> impl IntoResponse {
    render(DashboardProductCreateTemplate)
}

pub async fn dashboard_product_edit() -> impl IntoResponse {
    render(DashboardProductEditTemplate)
}

pub async fn dashboard_quote_requests() -> impl IntoResponse {
    render(DashboardQuoteRequestsTemplate)
}

pub async fn dashboard_faqs() -> impl IntoResponse {
    render(DashboardFaqsTemplate)
}

pub async fn dashboard_faq_create() -> impl IntoResponse {
    render(DashboardFaqCreateTemplate)
}

pub async fn dashboard_faq_edit() -> impl IntoResponse {
    render(DashboardFaqEditTemplate)
}

pub async fn dashboard_testimonials() -> impl IntoResponse {
    render(DashboardTestimonialsTemplate)
}

pub async fn dashboard_testimonial_create() -> impl IntoResponse {
    render(DashboardTestimonialCreateTemplate)
}

pub async fn dashboard_testimonial_edit() -> impl IntoResponse {
    render(DashboardTestimonialEditTemplate)
}

pub async fn dashboard_services() -> impl IntoResponse {
    render(DashboardServicesTemplate)
}

pub async fn dashboard_service_create() -> impl IntoResponse {
    render(DashboardServiceCreateTemplate)
}

pub async fn dashboard_service_edit() -> impl IntoResponse {
    render(DashboardServiceEditTemplate)
}

pub async fn dashboard_pages() -> impl IntoResponse {
    render(DashboardPagesTemplate)
}

pub async fn dashboard_seo() -> impl IntoResponse {
    render(DashboardSeoTemplate)
}

pub async fn dashboard_media() -> impl IntoResponse {
    render(DashboardMediaTemplate)
}

pub async fn dashboard_menus() -> impl IntoResponse {
    render(DashboardMenusTemplate)
}

pub async fn dashboard_settings() -> impl IntoResponse {
    render(DashboardSettingsTemplate)
}

pub async fn dashboard_users() -> impl IntoResponse {
    render(DashboardUsersTemplate)
}

pub async fn dashboard_audit_logs() -> impl IntoResponse {
    render(DashboardAuditLogsTemplate)
}
