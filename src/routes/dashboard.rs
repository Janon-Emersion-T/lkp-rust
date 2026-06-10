use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{
        dashboard, dashboard_audit_logs, dashboard_career_applications, dashboard_career_create,
        dashboard_career_edit, dashboard_careers, dashboard_contact_message_delete,
        dashboard_contact_message_quick_status, dashboard_contact_message_reply,
        dashboard_contact_message_show, dashboard_contact_message_update,
        dashboard_contact_messages, dashboard_faq_create, dashboard_faq_edit, dashboard_faqs,
        dashboard_industries, dashboard_industry_create, dashboard_industry_edit,
        dashboard_insight_categories, dashboard_insight_category_create,
        dashboard_insight_category_edit, dashboard_insight_create, dashboard_insight_edit,
        dashboard_insight_tag_create, dashboard_insight_tag_edit, dashboard_insight_tags,
        dashboard_insights, dashboard_leads, dashboard_media, dashboard_menus,
        dashboard_milestone_create, dashboard_milestone_edit, dashboard_milestones,
        dashboard_newsletter_subscribers, dashboard_pages, dashboard_portfolio_create,
        dashboard_portfolio_edit, dashboard_portfolios, dashboard_product_create,
        dashboard_product_edit, dashboard_products, dashboard_quote_requests, dashboard_seo,
        dashboard_service_create, dashboard_service_edit, dashboard_services, dashboard_settings,
        dashboard_testimonial_create, dashboard_testimonial_edit, dashboard_testimonials,
        dashboard_users,
    },
    state::AppState,
};

pub fn dashboard_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/dashboard/portfolios", get(dashboard_portfolios))
        .route(
            "/dashboard/portfolios/create",
            get(dashboard_portfolio_create),
        )
        .route(
            "/dashboard/portfolios/{id}/edit",
            get(dashboard_portfolio_edit),
        )
        .route("/dashboard/insights", get(dashboard_insights))
        .route("/dashboard/insights/create", get(dashboard_insight_create))
        .route("/dashboard/insights/{id}/edit", get(dashboard_insight_edit))
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
        .route("/dashboard/insight-tags", get(dashboard_insight_tags))
        .route(
            "/dashboard/insight-tags/create",
            get(dashboard_insight_tag_create),
        )
        .route(
            "/dashboard/insight-tags/{id}/edit",
            get(dashboard_insight_tag_edit),
        )
        .route("/dashboard/milestones", get(dashboard_milestones))
        .route(
            "/dashboard/milestones/create",
            get(dashboard_milestone_create),
        )
        .route(
            "/dashboard/milestones/{id}/edit",
            get(dashboard_milestone_edit),
        )
        .route("/dashboard/products", get(dashboard_products))
        .route("/dashboard/products/create", get(dashboard_product_create))
        .route("/dashboard/products/{id}/edit", get(dashboard_product_edit))
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
        .route("/dashboard/faqs", get(dashboard_faqs))
        .route("/dashboard/faqs/create", get(dashboard_faq_create))
        .route("/dashboard/faqs/{id}/edit", get(dashboard_faq_edit))
        .route("/dashboard/testimonials", get(dashboard_testimonials))
        .route(
            "/dashboard/testimonials/create",
            get(dashboard_testimonial_create),
        )
        .route(
            "/dashboard/testimonials/{id}/edit",
            get(dashboard_testimonial_edit),
        )
        .route("/dashboard/careers", get(dashboard_careers))
        .route("/dashboard/careers/create", get(dashboard_career_create))
        .route("/dashboard/careers/{id}/edit", get(dashboard_career_edit))
        .route(
            "/dashboard/career-applications",
            get(dashboard_career_applications),
        )
        .route("/dashboard/services", get(dashboard_services))
        .route("/dashboard/services/create", get(dashboard_service_create))
        .route("/dashboard/services/{id}/edit", get(dashboard_service_edit))
        .route("/dashboard/industries", get(dashboard_industries))
        .route(
            "/dashboard/industries/create",
            get(dashboard_industry_create),
        )
        .route(
            "/dashboard/industries/{id}/edit",
            get(dashboard_industry_edit),
        )
        .route("/dashboard/pages", get(dashboard_pages))
        .route("/dashboard/seo", get(dashboard_seo))
        .route("/dashboard/media", get(dashboard_media))
        .route("/dashboard/menus", get(dashboard_menus))
        .route("/dashboard/settings", get(dashboard_settings))
        .route("/dashboard/users", get(dashboard_users))
        .route("/dashboard/audit-logs", get(dashboard_audit_logs))
        .route(
            "/dashboard/contact-messages/{id}",
            get(dashboard_contact_message_show),
        )
        .route(
            "/dashboard/contact-messages/{id}/reply",
            post(dashboard_contact_message_reply),
        )
        .route(
            "/dashboard/contact-messages/{id}/update",
            post(dashboard_contact_message_update),
        )
        .route(
            "/dashboard/contact-messages/{id}/quick-status",
            post(dashboard_contact_message_quick_status),
        )
        .route(
            "/dashboard/contact-messages/{id}/delete",
            post(dashboard_contact_message_delete),
        )
}
