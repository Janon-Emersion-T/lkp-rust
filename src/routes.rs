use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

use crate::{
    auth::handlers::{logout, process_login, show_login},
    handlers::*,
    state::AppState,
};

pub fn app_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(show_login).post(process_login))
        .route("/logout", post(logout))
        /* =========================================
           PUBLIC PAGES
        ========================================= */
        .route("/", get(home))
        .route("/about", get(about))
        .route("/services", get(services))
        .route("/industries", get(industries))
        // Portfolio
        .route("/portfolio", get(portfolio))
        .route("/portfolio/{slug}", get(portfolio_single))
        // Insights
        .route("/insights", get(insights))
        .route("/insights/{slug}", get(insight_single))
        // Careers
        .route("/careers", get(careers))
        .route("/careers/{slug}", get(career_single))
        .route("/careers/{slug}/apply", get(career_apply))
        // Contact & Quote
        .route("/contact", get(contact))
        .route("/request-quote", get(request_quote))
        /* =========================================
           LEGAL PAGES
        ========================================= */
        .route("/terms-and-conditions", get(terms))
        .route("/service-level-agreement", get(sla))
        .route("/refund-policy", get(refund_policy))
        .route("/privacy-policy", get(privacy_policy))
        .route("/cookie-policy", get(cookie_policy))
        /* =========================================
           SERVICE DETAIL PAGES
        ========================================= */
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
        /* =========================================
           DASHBOARD
        ========================================= */
        .route("/dashboard", get(dashboard))
        /* =========================================
           DASHBOARD - PORTFOLIOS
        ========================================= */
        .route("/dashboard/portfolios", get(dashboard_portfolios))
        .route(
            "/dashboard/portfolios/create",
            get(dashboard_portfolio_create),
        )
        .route(
            "/dashboard/portfolios/{id}/edit",
            get(dashboard_portfolio_edit),
        )
        /* =========================================
           DASHBOARD - INSIGHTS
        ========================================= */
        .route("/dashboard/insights", get(dashboard_insights))
        .route("/dashboard/insights/create", get(dashboard_insight_create))
        .route("/dashboard/insights/{id}/edit", get(dashboard_insight_edit))
        /* =========================================
           DASHBOARD - INSIGHT CATEGORIES
        ========================================= */
        .route(
            "/dashboard/insight-categories",
            get(dashboard_insight_categories),
        )
        .route(
            "/dashboard/insight-categories/create",
            get(dashboard_insight_category_create),
        )
        .route(
            "/dashboard/insight-categories/{id}/edit",
            get(dashboard_insight_category_edit),
        )
        /* =========================================
           DASHBOARD - INSIGHT TAGS
        ========================================= */
        .route("/dashboard/insight-tags", get(dashboard_insight_tags))
        .route(
            "/dashboard/insight-tags/create",
            get(dashboard_insight_tag_create),
        )
        .route(
            "/dashboard/insight-tags/{id}/edit",
            get(dashboard_insight_tag_edit),
        )
        /* =========================================
           DASHBOARD - MILESTONES
        ========================================= */
        .route("/dashboard/milestones", get(dashboard_milestones))
        .route(
            "/dashboard/milestones/create",
            get(dashboard_milestone_create),
        )
        .route(
            "/dashboard/milestones/{id}/edit",
            get(dashboard_milestone_edit),
        )
        /* =========================================
           DASHBOARD - PRODUCTS
        ========================================= */
        .route("/dashboard/products", get(dashboard_products))
        .route("/dashboard/products/create", get(dashboard_product_create))
        .route("/dashboard/products/{id}/edit", get(dashboard_product_edit))
        /* =========================================
           DASHBOARD - LEADS & COMMUNICATION
        ========================================= */
        .route("/dashboard/leads", get(dashboard_leads))
        .route(
            "/dashboard/contact-messages",
            get(dashboard_contact_messages),
        )
        .route("/dashboard/quote-requests", get(dashboard_quote_requests))
        .route(
            "/dashboard/newsletter-subscribers",
            get(dashboard_newsletter_subscribers),
        )
        /* =========================================
           DASHBOARD - FAQS
        ========================================= */
        .route("/dashboard/faqs", get(dashboard_faqs))
        .route("/dashboard/faqs/create", get(dashboard_faq_create))
        .route("/dashboard/faqs/{id}/edit", get(dashboard_faq_edit))
        /* =========================================
           DASHBOARD - TESTIMONIALS
        ========================================= */
        .route("/dashboard/testimonials", get(dashboard_testimonials))
        .route(
            "/dashboard/testimonials/create",
            get(dashboard_testimonial_create),
        )
        .route(
            "/dashboard/testimonials/{id}/edit",
            get(dashboard_testimonial_edit),
        )
        /* =========================================
           DASHBOARD - CAREERS
        ========================================= */
        .route("/dashboard/careers", get(dashboard_careers))
        .route("/dashboard/careers/create", get(dashboard_career_create))
        .route("/dashboard/careers/{id}/edit", get(dashboard_career_edit))
        .route(
            "/dashboard/career-applications",
            get(dashboard_career_applications),
        )
        /* =========================================
           DASHBOARD - SERVICES
        ========================================= */
        .route("/dashboard/services", get(dashboard_services))
        .route("/dashboard/services/create", get(dashboard_service_create))
        .route("/dashboard/services/{id}/edit", get(dashboard_service_edit))
        /* =========================================
           DASHBOARD - INDUSTRIES
        ========================================= */
        .route("/dashboard/industries", get(dashboard_industries))
        .route(
            "/dashboard/industries/create",
            get(dashboard_industry_create),
        )
        .route(
            "/dashboard/industries/{id}/edit",
            get(dashboard_industry_edit),
        )
        /* =========================================
           DASHBOARD - SYSTEM
        ========================================= */
        .route("/dashboard/pages", get(dashboard_pages))
        .route("/dashboard/seo", get(dashboard_seo))
        .route("/dashboard/media", get(dashboard_media))
        .route("/dashboard/menus", get(dashboard_menus))
        .route("/dashboard/settings", get(dashboard_settings))
        .route("/dashboard/users", get(dashboard_users))
        .route("/dashboard/audit-logs", get(dashboard_audit_logs))
        /* =========================================
           STATIC FILES
        ========================================= */
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
