use axum::response::IntoResponse;

use super::{
    render::render,
    templates::{
        DashboardAuditLogsTemplate, DashboardCareerApplicationsTemplate,
        DashboardCareerCreateTemplate, DashboardCareerEditTemplate, DashboardCareersTemplate,
        DashboardContactMessagesTemplate, DashboardFaqCreateTemplate, DashboardFaqEditTemplate,
        DashboardFaqsTemplate, DashboardIndustriesTemplate, DashboardIndustryCreateTemplate,
        DashboardIndustryEditTemplate, DashboardInsightCategoriesTemplate,
        DashboardInsightCategoryCreateTemplate, DashboardInsightCategoryEditTemplate,
        DashboardInsightCreateTemplate, DashboardInsightEditTemplate,
        DashboardInsightTagCreateTemplate, DashboardInsightTagEditTemplate,
        DashboardInsightTagsTemplate, DashboardInsightsTemplate, DashboardLeadsTemplate,
        DashboardMediaTemplate, DashboardMenusTemplate, DashboardMilestoneCreateTemplate,
        DashboardMilestoneEditTemplate, DashboardMilestonesTemplate,
        DashboardNewsletterSubscribersTemplate, DashboardPagesTemplate,
        DashboardPortfolioCreateTemplate, DashboardPortfolioEditTemplate,
        DashboardPortfoliosTemplate, DashboardProductCreateTemplate, DashboardProductEditTemplate,
        DashboardProductsTemplate, DashboardQuoteRequestsTemplate, DashboardSeoTemplate,
        DashboardServiceCreateTemplate, DashboardServiceEditTemplate, DashboardServicesTemplate,
        DashboardSettingsTemplate, DashboardTemplate, DashboardTestimonialCreateTemplate,
        DashboardTestimonialEditTemplate, DashboardTestimonialsTemplate, DashboardUsersTemplate,
    },
};

pub async fn dashboard() -> impl IntoResponse {
    render(DashboardTemplate)
}

pub async fn dashboard_portfolios() -> impl IntoResponse {
    render(DashboardPortfoliosTemplate)
}

pub async fn dashboard_portfolio_create() -> impl IntoResponse {
    render(DashboardPortfolioCreateTemplate)
}

pub async fn dashboard_portfolio_edit() -> impl IntoResponse {
    render(DashboardPortfolioEditTemplate)
}

pub async fn dashboard_insights() -> impl IntoResponse {
    render(DashboardInsightsTemplate)
}

pub async fn dashboard_insight_create() -> impl IntoResponse {
    render(DashboardInsightCreateTemplate)
}

pub async fn dashboard_insight_edit() -> impl IntoResponse {
    render(DashboardInsightEditTemplate)
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

pub async fn dashboard_leads() -> impl IntoResponse {
    render(DashboardLeadsTemplate)
}

pub async fn dashboard_contact_messages() -> impl IntoResponse {
    render(DashboardContactMessagesTemplate)
}

pub async fn dashboard_quote_requests() -> impl IntoResponse {
    render(DashboardQuoteRequestsTemplate)
}

pub async fn dashboard_newsletter_subscribers() -> impl IntoResponse {
    render(DashboardNewsletterSubscribersTemplate)
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

pub async fn dashboard_careers() -> impl IntoResponse {
    render(DashboardCareersTemplate)
}

pub async fn dashboard_career_create() -> impl IntoResponse {
    render(DashboardCareerCreateTemplate)
}

pub async fn dashboard_career_edit() -> impl IntoResponse {
    render(DashboardCareerEditTemplate)
}

pub async fn dashboard_career_applications() -> impl IntoResponse {
    render(DashboardCareerApplicationsTemplate)
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

pub async fn dashboard_industries() -> impl IntoResponse {
    render(DashboardIndustriesTemplate)
}

pub async fn dashboard_industry_create() -> impl IntoResponse {
    render(DashboardIndustryCreateTemplate)
}

pub async fn dashboard_industry_edit() -> impl IntoResponse {
    render(DashboardIndustryEditTemplate)
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
