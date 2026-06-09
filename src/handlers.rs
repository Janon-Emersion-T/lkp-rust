use askama::Template;
use axum::{
    extract::{Path, State, Form},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

use serde::Deserialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::state::AppState;

pub fn render<T: Template>(template: T) -> impl IntoResponse {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(error) => {
            eprintln!("Template error: {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Template rendering error",
            )
                .into_response()
        }
    }
}

macro_rules! page {
    ($name:ident, $path:literal) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name;
    };
}

// Public pages
page!(HomeTemplate, "pages/home.html");
page!(AboutTemplate, "pages/aboutus.html");
page!(ServicesTemplate, "pages/services.html");
page!(IndustriesTemplate, "pages/industries.html");
page!(PortfolioTemplate, "pages/portfolio.html");
page!(InsightsTemplate, "pages/insights.html");
page!(CareersTemplate, "pages/careers.html");
page!(ContactTemplate, "pages/contactus.html");
page!(RequestQuoteTemplate, "pages/requestquote.html");

page!(TermsTemplate, "pages/termsandconditions.html");
page!(SlaTemplate, "pages/sla.html");
page!(RefundPolicyTemplate, "pages/refundpolicy.html");
page!(PrivacyPolicyTemplate, "pages/privacypolicy.html");
page!(CookiePolicyTemplate, "pages/cookiepolicy.html");

// Service pages
page!(
    WebDevelopmentTemplate,
    "pages/services/web-development.html"
);
page!(
    MobileAppDevelopmentTemplate,
    "pages/services/mobile-app-development.html"
);
page!(
    CustomSoftwareDevelopmentTemplate,
    "pages/services/custom-software-development.html"
);
page!(
    SoftwareDevelopmentTemplate,
    "pages/services/software-development.html"
);
page!(
    DigitalMarketingTemplate,
    "pages/services/digital-marketing.html"
);
page!(
    SeoSearchGrowthTemplate,
    "pages/services/seo-search-growth.html"
);
page!(
    HostingDomainCloudTemplate,
    "pages/services/hosting-domain-cloud.html"
);
page!(AiAutomationTemplate, "pages/services/ai-automation.html");
page!(
    ItConsultationTemplate,
    "pages/services/it-consultation.html"
);

// Dynamic public pages
#[derive(Template)]
#[template(path = "pages/portfolios/single.html")]
pub struct PortfolioSingleTemplate {
    pub slug: String,
}

#[derive(Template)]
#[template(path = "pages/insights/single.html")]
pub struct InsightSingleTemplate {
    pub slug: String,
}

#[derive(Template)]
#[template(path = "pages/careers/single.html")]
pub struct CareerSingleTemplate {
    pub slug: String,
}

#[derive(Template)]
#[template(path = "pages/careers/apply.html")]
pub struct CareerApplyTemplate {
    pub slug: String,
}

#[derive(Deserialize)]
pub struct ContactMessageForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ContactMessage {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub subject: String,
    pub message: String,
    pub status: String,
    pub admin_reply: Option<String>,
    pub replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ReplyForm {
    pub admin_reply: String,
}

// Dashboard
page!(DashboardTemplate, "dashboard/index.html");

page!(
    DashboardPortfoliosTemplate,
    "dashboard/portfolios/index.html"
);
page!(
    DashboardPortfolioCreateTemplate,
    "dashboard/portfolios/create.html"
);
page!(
    DashboardPortfolioEditTemplate,
    "dashboard/portfolios/edit.html"
);

page!(DashboardInsightsTemplate, "dashboard/insights/index.html");
page!(
    DashboardInsightCreateTemplate,
    "dashboard/insights/create.html"
);
page!(DashboardInsightEditTemplate, "dashboard/insights/edit.html");

page!(
    DashboardInsightCategoriesTemplate,
    "dashboard/insight-categories/index.html"
);
page!(
    DashboardInsightCategoryCreateTemplate,
    "dashboard/insight-categories/create.html"
);
page!(
    DashboardInsightCategoryEditTemplate,
    "dashboard/insight-categories/edit.html"
);

page!(
    DashboardInsightTagsTemplate,
    "dashboard/insight-tags/index.html"
);
page!(
    DashboardInsightTagCreateTemplate,
    "dashboard/insight-tags/create.html"
);
page!(
    DashboardInsightTagEditTemplate,
    "dashboard/insight-tags/edit.html"
);

page!(
    DashboardMilestonesTemplate,
    "dashboard/milestones/index.html"
);
page!(
    DashboardMilestoneCreateTemplate,
    "dashboard/milestones/create.html"
);
page!(
    DashboardMilestoneEditTemplate,
    "dashboard/milestones/edit.html"
);

page!(DashboardProductsTemplate, "dashboard/products/index.html");
page!(
    DashboardProductCreateTemplate,
    "dashboard/products/create.html"
);
page!(DashboardProductEditTemplate, "dashboard/products/edit.html");

page!(DashboardLeadsTemplate, "dashboard/leads/index.html");
page!(
    DashboardContactMessagesTemplate,
    "dashboard/contact-messages/index.html"
);
page!(
    DashboardQuoteRequestsTemplate,
    "dashboard/quote-requests/index.html"
);
page!(
    DashboardNewsletterSubscribersTemplate,
    "dashboard/newsletter-subscribers/index.html"
);

page!(DashboardFaqsTemplate, "dashboard/faqs/index.html");
page!(DashboardFaqCreateTemplate, "dashboard/faqs/create.html");
page!(DashboardFaqEditTemplate, "dashboard/faqs/edit.html");

page!(
    DashboardTestimonialsTemplate,
    "dashboard/testimonials/index.html"
);
page!(
    DashboardTestimonialCreateTemplate,
    "dashboard/testimonials/create.html"
);
page!(
    DashboardTestimonialEditTemplate,
    "dashboard/testimonials/edit.html"
);

page!(DashboardCareersTemplate, "dashboard/careers/index.html");
page!(
    DashboardCareerCreateTemplate,
    "dashboard/careers/create.html"
);
page!(DashboardCareerEditTemplate, "dashboard/careers/edit.html");
page!(
    DashboardCareerApplicationsTemplate,
    "dashboard/career-applications/index.html"
);

page!(DashboardServicesTemplate, "dashboard/services/index.html");
page!(
    DashboardServiceCreateTemplate,
    "dashboard/services/create.html"
);
page!(DashboardServiceEditTemplate, "dashboard/services/edit.html");

page!(
    DashboardIndustriesTemplate,
    "dashboard/industries/index.html"
);
page!(
    DashboardIndustryCreateTemplate,
    "dashboard/industries/create.html"
);
page!(
    DashboardIndustryEditTemplate,
    "dashboard/industries/edit.html"
);

page!(DashboardPagesTemplate, "dashboard/pages/index.html");
page!(DashboardSeoTemplate, "dashboard/seo/index.html");
page!(DashboardMediaTemplate, "dashboard/media/index.html");
page!(DashboardMenusTemplate, "dashboard/menus/index.html");
page!(DashboardSettingsTemplate, "dashboard/settings/index.html");
page!(DashboardUsersTemplate, "dashboard/users/index.html");
page!(
    DashboardAuditLogsTemplate,
    "dashboard/audit-logs/index.html"
);

// Public handlers
pub async fn submit_contact_message(
    State(state): State<AppState>,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        INSERT INTO contact_messages
        (name, email, phone, company, subject, message)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(form.name.trim())
    .bind(form.email.trim().to_lowercase())
    .bind(form.phone)
    .bind(form.company)
    .bind(form.subject.trim())
    .bind(form.message.trim())
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Redirect::to("/contact?success=1").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to submit contact message",
        ).into_response(),
    }
}

pub async fn dashboard_contact_message_show(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let _id = id;

    render(DashboardContactMessagesTemplate)
}

pub async fn dashboard_contact_message_reply(
    Path(id): Path<Uuid>,
    Form(form): Form<ReplyForm>,
) -> impl IntoResponse {
    let _id = id;
    let _reply = form.admin_reply;

    Redirect::to("/dashboard/contact-messages")
}

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
pub async fn contact() -> impl IntoResponse {
    render(ContactTemplate)
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

pub async fn terms() -> impl IntoResponse {
    render(TermsTemplate)
}
pub async fn sla() -> impl IntoResponse {
    render(SlaTemplate)
}
pub async fn refund_policy() -> impl IntoResponse {
    render(RefundPolicyTemplate)
}
pub async fn privacy_policy() -> impl IntoResponse {
    render(PrivacyPolicyTemplate)
}
pub async fn cookie_policy() -> impl IntoResponse {
    render(CookiePolicyTemplate)
}

// Service handlers
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

// Dashboard handlers
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
