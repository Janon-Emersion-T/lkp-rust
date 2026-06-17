use askama::Template;

use crate::{
    handlers::service_content::{ServiceCard, ServicePageContext, ServicePoint, ServiceStep},
    models::{
        CareerApplicationEditorView, CareerApplicationRecord, CareerCardView, CareerDetailView,
        CareerEditorView, CareerRecord, ContactMessage, IndustryCardView, IndustryEditorView,
        IndustryRecord, InsightCardView, InsightDetailView, InsightEditorView, InsightRecord,
        LeadStats, NewsletterCampaignEditorView, NewsletterCampaignRecord,
        NewsletterSubscriberEditorView, NewsletterSubscriberRecord, PortfolioCardView,
        PortfolioDetailView, PortfolioEditorView, PortfolioRecord, ServiceAreaEditorView,
        ServiceAreaRecord,
    },
};

macro_rules! page {
    ($name:ident, $path:literal) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name;
    };
}

// Public pages
#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub featured_portfolios: Vec<PortfolioCardView>,
    pub featured_insights: Vec<InsightCardView>,
    pub featured_industries: Vec<IndustryCardView>,
}
page!(AboutTemplate, "pages/aboutus.html");
#[derive(Template)]
#[template(path = "pages/services.html")]
pub struct ServicesTemplate {
    pub services: Vec<ServiceCard>,
    pub proof_points: Vec<ServicePoint>,
    pub process: Vec<ServiceStep>,
}
#[derive(Template)]
#[template(path = "pages/industries.html")]
pub struct IndustriesTemplate {
    pub industries: Vec<IndustryCardView>,
    pub total_count: usize,
    pub featured_count: usize,
}
#[derive(Template)]
#[template(path = "pages/portfolio.html")]
pub struct CaseStudiesTemplate {
    pub portfolios: Vec<PortfolioCardView>,
    pub featured_portfolios: Vec<PortfolioCardView>,
    pub total_count: usize,
    pub featured_count: usize,
    pub industry_count: usize,
}
#[derive(Template)]
#[template(path = "pages/insights.html")]
pub struct InsightsTemplate {
    pub insights: Vec<InsightCardView>,
    pub featured_insights: Vec<InsightCardView>,
    pub categories: Vec<String>,
    pub snapshot_metrics: Vec<InsightSnapshotMetric>,
    pub pagination: PaginationView,
}

#[derive(Debug, Clone)]
pub struct InsightSnapshotMetric {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct PaginationView {
    pub current_page: usize,
    pub total_pages: usize,
    pub previous_page_url: Option<String>,
    pub next_page_url: Option<String>,
    pub page_links: Vec<PaginationLink>,
}

#[derive(Debug, Clone)]
pub struct PaginationLink {
    pub label: String,
    pub url: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct SitemapLinkView {
    pub title: String,
    pub url: String,
    pub description: String,
    pub has_lastmod: bool,
    pub lastmod: Option<String>,
    pub lastmod_label: String,
}

#[derive(Debug, Clone)]
pub struct SitemapSectionView {
    pub title: String,
    pub description: String,
    pub link_count: usize,
    pub links: Vec<SitemapLinkView>,
}

#[derive(Debug, Clone)]
pub struct ServiceAreaCardView {
    pub region: String,
    pub title: String,
    pub path: String,
    pub summary: String,
    pub area_type_label: String,
}

#[derive(Debug, Clone)]
pub struct ServiceAreaGroupView {
    pub title: String,
    pub description: String,
    pub areas: Vec<ServiceAreaCardView>,
}

#[derive(Debug, Clone)]
pub struct ServiceAreaDetailPointView {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ServiceAreaDetailFaqView {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone)]
pub struct ServiceAreaPageView {
    pub city: String,
    pub region: String,
    pub title: String,
    pub canonical_path: String,
    pub meta_title: String,
    pub meta_description: String,
    pub hero_title: String,
    pub hero_description: String,
    pub positioning: String,
    pub timezone_note: String,
    pub nearby_markets: Vec<String>,
    pub buyer_points: Vec<ServiceAreaDetailPointView>,
    pub service_points: Vec<ServiceAreaDetailPointView>,
    pub faqs: Vec<ServiceAreaDetailFaqView>,
    pub image_gallery: Vec<String>,
    pub area_type_label: String,
}

#[derive(Template)]
#[template(path = "pages/careers.html")]
pub struct CareersTemplate {
    pub careers: Vec<CareerCardView>,
    pub featured_roles: Vec<CareerCardView>,
    pub open_roles: usize,
    pub teams: Vec<String>,
}

page!(WhyWorkTemplate, "pages/careers/why-work.html");
page!(FaqTemplate, "pages/faq.html");
#[derive(Template)]
#[template(path = "pages/contactus.html")]
pub struct ContactTemplate {
    pub success: bool,
}
page!(RequestQuoteTemplate, "pages/requestquote.html");
#[derive(Template)]
#[template(path = "pages/service-areas.html")]
pub struct ServiceAreasTemplate {
    pub groups: Vec<ServiceAreaGroupView>,
    pub all_areas: Vec<ServiceAreaCardView>,
    pub total_areas: usize,
    pub featured_services: Vec<ServiceCard>,
}
#[derive(Template)]
#[template(path = "pages/service-areas/detail.html")]
pub struct ServiceAreaDetailTemplate {
    pub page: ServiceAreaPageView,
    pub related_areas: Vec<ServiceAreaCardView>,
    pub featured_services: Vec<ServiceCard>,
}
#[derive(Template)]
#[template(path = "pages/sitemap.html")]
pub struct SitemapTemplate {
    pub sections: Vec<SitemapSectionView>,
    pub total_urls: usize,
}

// Legal pages
page!(TermsTemplate, "pages/termsandconditions.html");
page!(SlaTemplate, "pages/sla.html");
page!(RefundPolicyTemplate, "pages/refundpolicy.html");
page!(PrivacyPolicyTemplate, "pages/privacypolicy.html");
page!(CookiePolicyTemplate, "pages/cookiepolicy.html");

// Service pages
#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct WebDevelopmentTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct MobileAppDevelopmentTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct CustomSoftwareDevelopmentTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct SoftwareDevelopmentTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct DigitalMarketingTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct SeoSearchGrowthTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct HostingDomainCloudTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct AiAutomationTemplate {
    pub page: ServicePageContext,
}

#[derive(Template)]
#[template(path = "pages/services/detail.html")]
pub struct ItConsultationTemplate {
    pub page: ServicePageContext,
}

// Dynamic public pages
#[derive(Template)]
#[template(path = "pages/portfolios/single.html")]
pub struct PortfolioSingleTemplate {
    pub portfolio: PortfolioDetailView,
    pub related: Vec<PortfolioCardView>,
}

#[derive(Template)]
#[template(path = "pages/insights/single.html")]
pub struct InsightSingleTemplate {
    pub insight: InsightDetailView,
    pub related: Vec<InsightCardView>,
}

#[derive(Template)]
#[template(path = "pages/careers/single.html")]
pub struct CareerSingleTemplate {
    pub career: CareerDetailView,
    pub related: Vec<CareerCardView>,
}

#[derive(Template)]
#[template(path = "pages/careers/apply.html")]
pub struct CareerApplyTemplate {
    pub career: CareerDetailView,
    pub form: CareerApplicationEditorView,
    pub success: bool,
}

// Dashboard
page!(DashboardTemplate, "dashboard/index.html");

#[derive(Template)]
#[template(path = "dashboard/portfolios/index.html")]
pub struct DashboardPortfoliosTemplate {
    pub portfolios: Vec<PortfolioRecord>,
    pub saved: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/portfolios/create.html")]
pub struct DashboardPortfolioCreateTemplate {
    pub portfolio: PortfolioEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/portfolios/edit.html")]
pub struct DashboardPortfolioEditTemplate {
    pub portfolio: PortfolioEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/insights/index.html")]
pub struct DashboardInsightsTemplate {
    pub insights: Vec<InsightRecord>,
    pub saved: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/insights/create.html")]
pub struct DashboardInsightCreateTemplate {
    pub insight: InsightEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/insights/edit.html")]
pub struct DashboardInsightEditTemplate {
    pub insight: InsightEditorView,
    pub action_url: String,
}

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
#[derive(Template)]
#[template(path = "dashboard/newsletter-subscribers/index.html")]
pub struct DashboardNewsletterSubscribersTemplate {
    pub subscribers: Vec<NewsletterSubscriberRecord>,
    pub subscriber: NewsletterSubscriberEditorView,
    pub action_url: String,
    pub saved: bool,
    pub deleted: bool,
    pub imported_count: usize,
}

#[derive(Template)]
#[template(path = "dashboard/newsletter-subscribers/edit.html")]
pub struct DashboardNewsletterSubscriberEditTemplate {
    pub subscriber: NewsletterSubscriberEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/newsletters/index.html")]
pub struct DashboardNewslettersTemplate {
    pub campaigns: Vec<NewsletterCampaignRecord>,
    pub campaign: NewsletterCampaignEditorView,
    pub action_url: String,
    pub saved: bool,
    pub deleted: bool,
}

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

#[derive(Template)]
#[template(path = "dashboard/careers/index.html")]
pub struct DashboardCareersTemplate {
    pub careers: Vec<CareerRecord>,
    pub saved: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/careers/create.html")]
pub struct DashboardCareerCreateTemplate {
    pub career: CareerEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/careers/edit.html")]
pub struct DashboardCareerEditTemplate {
    pub career: CareerEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/career-applications/index.html")]
pub struct DashboardCareerApplicationsTemplate {
    pub applications: Vec<CareerApplicationRecord>,
    pub total_count: usize,
    pub new_count: usize,
    pub shortlisted_count: usize,
    pub updated: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/career-applications/show.html")]
pub struct CareerApplicationShowTemplate {
    pub application: CareerApplicationRecord,
}

page!(DashboardServicesTemplate, "dashboard/services/index.html");
page!(
    DashboardServiceCreateTemplate,
    "dashboard/services/create.html"
);
page!(DashboardServiceEditTemplate, "dashboard/services/edit.html");

#[derive(Template)]
#[template(path = "dashboard/industries/index.html")]
pub struct DashboardIndustriesTemplate {
    pub industries: Vec<IndustryRecord>,
    pub saved: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/industries/create.html")]
pub struct DashboardIndustryCreateTemplate {
    pub industry: IndustryEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/industries/edit.html")]
pub struct DashboardIndustryEditTemplate {
    pub industry: IndustryEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/service-areas/index.html")]
pub struct DashboardServiceAreasTemplate {
    pub service_areas: Vec<ServiceAreaRecord>,
    pub saved: bool,
    pub deleted: bool,
}

#[derive(Template)]
#[template(path = "dashboard/service-areas/create.html")]
pub struct DashboardServiceAreaCreateTemplate {
    pub service_area: ServiceAreaEditorView,
    pub action_url: String,
}

#[derive(Template)]
#[template(path = "dashboard/service-areas/edit.html")]
pub struct DashboardServiceAreaEditTemplate {
    pub service_area: ServiceAreaEditorView,
    pub action_url: String,
}

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
