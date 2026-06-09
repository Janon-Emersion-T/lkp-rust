use axum::{Router, routing::get};

use crate::{
    handlers::{cookie_policy, privacy_policy, refund_policy, sla, terms},
    state::AppState,
};

pub fn legal_routes() -> Router<AppState> {
    Router::new()
        .route("/terms-and-conditions", get(terms))
        .route("/service-level-agreement", get(sla))
        .route("/refund-policy", get(refund_policy))
        .route("/privacy-policy", get(privacy_policy))
        .route("/cookie-policy", get(cookie_policy))
}
