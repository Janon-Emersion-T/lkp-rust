use axum::response::IntoResponse;

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
