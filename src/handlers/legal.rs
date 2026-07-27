use axum::response::{IntoResponse, Redirect};

use super::{
    render::render,
    templates::{
        CookiePolicyTemplate, PrivacyPolicyTemplate, RefundPolicyTemplate, SlaTemplate,
        TermsTemplate,
    },
};

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

pub async fn terms_alias() -> impl IntoResponse {
    Redirect::permanent("/terms-and-conditions")
}

pub async fn sla_alias() -> impl IntoResponse {
    Redirect::permanent("/service-level-agreement")
}

pub async fn refund_policy_alias() -> impl IntoResponse {
    Redirect::permanent("/refund-policy")
}

pub async fn privacy_policy_alias() -> impl IntoResponse {
    Redirect::permanent("/privacy-policy")
}

pub async fn cookie_policy_alias() -> impl IntoResponse {
    Redirect::permanent("/cookie-policy")
}
