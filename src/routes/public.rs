use axum::{Router, routing::get};

use crate::{
    handlers::{
        about, career_apply, career_single, careers, contact, home, industries, insight_single,
        insights, portfolio, portfolio_single, request_quote, services, submit_contact_message,
    },
    state::AppState,
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/services", get(services))
        .route("/industries", get(industries))
        .route("/portfolio", get(portfolio))
        .route("/portfolio/{slug}", get(portfolio_single))
        .route("/insights", get(insights))
        .route("/insights/{slug}", get(insight_single))
        .route("/careers", get(careers))
        .route("/careers/{slug}", get(career_single))
        .route("/careers/{slug}/apply", get(career_apply))
        .route("/contact", get(contact).post(submit_contact_message))
        .route("/request-quote", get(request_quote))
}
