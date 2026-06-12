use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{
        about, career_apply, career_single, careers, case_studies, case_study_single, contact, faq,
        home, industries, insight_single, insights, request_quote, services,
        submit_career_application, submit_contact_message, subscribe_newsletter, why_work,
    },
    state::AppState,
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/services", get(services))
        .route("/industries", get(industries))
        .route("/portfolio", get(case_studies))
        .route("/portfolio/{slug}", get(case_study_single))
        .route("/case-studies", get(case_studies))
        .route("/case-studies/{slug}", get(case_study_single))
        .route("/insights", get(insights))
        .route("/insights/{slug}", get(insight_single))
        .route("/newsletter/subscribe", post(subscribe_newsletter))
        .route("/careers", get(careers))
        .route("/careers/why-work-at-lkprofessionals", get(why_work))
        .route("/careers/{slug}", get(career_single))
        .route(
            "/careers/{slug}/apply",
            get(career_apply).post(submit_career_application),
        )
        .route("/contact", get(contact).post(submit_contact_message))
        .route("/faq", get(faq))
        .route("/request-quote", get(request_quote))
}
