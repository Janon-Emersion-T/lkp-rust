use axum::{Router, routing::get};

use crate::{
    handlers::{
        cookie_policy, cookie_policy_alias, privacy_policy, privacy_policy_alias, refund_policy,
        refund_policy_alias, sla, sla_alias, terms, terms_alias,
    },
    state::AppState,
};

pub fn legal_routes() -> Router<AppState> {
    Router::new()
        .route("/terms-and-conditions", get(terms))
        .route("/termsandconditions", get(terms_alias))
        .route("/service-level-agreement", get(sla))
        .route("/sla", get(sla_alias))
        .route("/refund-policy", get(refund_policy))
        .route("/refundpolicy", get(refund_policy_alias))
        .route("/privacy-policy", get(privacy_policy))
        .route("/privacypolicy", get(privacy_policy_alias))
        .route("/cookie-policy", get(cookie_policy))
        .route("/cookiepolicy", get(cookie_policy_alias))
}
