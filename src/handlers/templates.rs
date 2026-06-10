use askama::Template;

use crate::models::{ContactMessage, LeadStats};

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
#[derive(Template)]
#[template(path = "pages/contactus.html")]
pub struct ContactTemplate {
    pub success: bool,
}
page!(RequestQuoteTemplate, "pages/requestquote.html");

// Legal pages
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

#[derive(Template)]
#[template(path = "dashboard/leads/index.html")]
pub struct DashboardLeadsTemplate {
    pub leads: Vec<ContactMessage>,
    pub stats: LeadStats,
    pub filters: crate::models::LeadFilters,
}

#[derive(Template)]
#[template(path = "dashboard/contact-messages/index.html")]
pub struct DashboardContactMessagesTemplate {
    pub messages: Vec<ContactMessage>,
    pub stats: LeadStats,
    pub filters: crate::models::LeadFilters,
}

#[derive(Template)]
#[template(path = "dashboard/contact-messages/show.html")]
pub struct DashboardContactMessageShowTemplate {
    pub message: ContactMessage,
}

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
