use axum::{
    middleware,
    Router,
    routing::{get, post},
};

use crate::{
    auth::middleware::require_auth,
    handlers::{
        dashboard, dashboard_audit_logs, dashboard_career_application_delete,
        dashboard_career_application_show, dashboard_career_application_update,
        dashboard_career_applications, dashboard_career_create, dashboard_career_delete,
        dashboard_career_edit, dashboard_career_store, dashboard_career_update, dashboard_careers,
        dashboard_contact_message_delete, dashboard_contact_message_quick_status,
        dashboard_contact_message_reply, dashboard_contact_message_show,
        dashboard_contact_message_update, dashboard_contact_messages, dashboard_faq_create,
        dashboard_faq_edit, dashboard_faqs, dashboard_industries, dashboard_industry_create,
        dashboard_industry_delete, dashboard_industry_edit, dashboard_industry_store,
        dashboard_industry_update, dashboard_insight_categories, dashboard_insight_category_create,
        dashboard_insight_category_edit, dashboard_insight_create, dashboard_insight_delete,
        dashboard_insight_edit, dashboard_insight_store, dashboard_insight_tag_create,
        dashboard_insight_tag_edit, dashboard_insight_tags, dashboard_insight_update,
        dashboard_insights, dashboard_leads, dashboard_media, dashboard_menus,
        dashboard_milestone_create, dashboard_milestone_edit, dashboard_milestones,
        dashboard_newsletter_delete, dashboard_newsletter_store,
        dashboard_newsletter_subscriber_bulk_import, dashboard_newsletter_subscriber_delete,
        dashboard_newsletter_subscriber_edit, dashboard_newsletter_subscriber_store,
        dashboard_newsletter_subscriber_update, dashboard_newsletter_subscribers,
        dashboard_newsletters, dashboard_pages, dashboard_portfolio_create,
        dashboard_portfolio_delete, dashboard_portfolio_edit, dashboard_portfolio_store,
        dashboard_portfolio_update, dashboard_portfolios, dashboard_product_create,
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
        .route(
            "/dashboard/portfolios",
            get(dashboard_portfolios).post(dashboard_portfolio_store),
        )
        .route(
            "/dashboard/portfolios/create",
            get(dashboard_portfolio_create),
        )
        .route(
            "/dashboard/portfolios/{id}/edit",
            get(dashboard_portfolio_edit).post(dashboard_portfolio_update),
        )
        .route(
            "/dashboard/portfolios/{id}/delete",
            post(dashboard_portfolio_delete),
        )
        .route(
            "/dashboard/insights",
            get(dashboard_insights).post(dashboard_insight_store),
        )
        .route("/dashboard/insights/create", get(dashboard_insight_create))
        .route(
            "/dashboard/insights/{id}/edit",
            get(dashboard_insight_edit).post(dashboard_insight_update),
        )
        .route(
            "/dashboard/insights/{id}/delete",
            post(dashboard_insight_delete),
        )
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
            get(dashboard_newsletter_subscribers).post(dashboard_newsletter_subscriber_store),
        )
        .route(
            "/dashboard/newsletter-subscribers/bulk-import",
            post(dashboard_newsletter_subscriber_bulk_import),
        )
        .route(
            "/dashboard/newsletter-subscribers/{id}/edit",
            get(dashboard_newsletter_subscriber_edit).post(dashboard_newsletter_subscriber_update),
        )
        .route(
            "/dashboard/newsletter-subscribers/{id}/delete",
            post(dashboard_newsletter_subscriber_delete),
        )
        .route(
            "/dashboard/newsletters",
            get(dashboard_newsletters).post(dashboard_newsletter_store),
        )
        .route(
            "/dashboard/newsletters/{id}/delete",
            post(dashboard_newsletter_delete),
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
        .route(
            "/dashboard/careers",
            get(dashboard_careers).post(dashboard_career_store),
        )
        .route("/dashboard/careers/create", get(dashboard_career_create))
        .route(
            "/dashboard/careers/{id}/edit",
            get(dashboard_career_edit).post(dashboard_career_update),
        )
        .route(
            "/dashboard/careers/{id}/delete",
            post(dashboard_career_delete),
        )
        .route(
            "/dashboard/career-applications",
            get(dashboard_career_applications),
        )
        .route(
            "/dashboard/career-applications/{id}",
            get(dashboard_career_application_show),
        )
        .route(
            "/dashboard/career-applications/{id}/update",
            post(dashboard_career_application_update),
        )
        .route(
            "/dashboard/career-applications/{id}/delete",
            post(dashboard_career_application_delete),
        )
        .route("/dashboard/services", get(dashboard_services))
        .route("/dashboard/services/create", get(dashboard_service_create))
        .route("/dashboard/services/{id}/edit", get(dashboard_service_edit))
        .route(
            "/dashboard/industries",
            get(dashboard_industries).post(dashboard_industry_store),
        )
        .route(
            "/dashboard/industries/create",
            get(dashboard_industry_create),
        )
        .route(
            "/dashboard/industries/{id}/edit",
            get(dashboard_industry_edit).post(dashboard_industry_update),
        )
        .route(
            "/dashboard/industries/{id}/delete",
            post(dashboard_industry_delete),
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
        .route_layer(middleware::from_fn(require_auth))
}
