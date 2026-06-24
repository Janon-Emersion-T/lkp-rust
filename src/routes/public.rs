use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{
        about, business_website_package, career_apply, career_single, careers, case_studies,
        case_study_single, comparison_page, contact, faq, founder_portfolio, free_audit_page, home,
        industries, insight_single, insights, insights_by_category, llmo_txt, llms_txt, packages,
        pricing_page, region_page, request_quote, robot_txt, robots_txt, service_area_single,
        service_areas, services, sitemap_html, sitemap_xml, solution_page,
        submit_career_application, submit_contact_message, submit_request_quote,
        subscribe_newsletter, why_work,
    },
    state::AppState,
};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/robots.txt", get(robots_txt))
        .route("/robot.txt", get(robot_txt))
        .route("/llms.txt", get(llms_txt))
        .route("/llmo.txt", get(llmo_txt))
        .route("/sitemap.html", get(sitemap_html))
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/", get(home))
        .route("/about", get(about))
        .route("/founder/janon-emersion-t", get(founder_portfolio))
        .route("/services", get(services))
        .route("/packages", get(packages))
        .route(
            "/packages/business-website-package",
            get(business_website_package),
        )
        .route("/solutions/{slug}", get(solution_page))
        .route("/pricing/{slug}", get(pricing_page))
        .route("/compare/{slug}", get(comparison_page))
        .route("/regions/{slug}", get(region_page))
        .route("/free-website-seo-audit", get(free_audit_page))
        .route("/service-area", get(service_areas))
        .route("/service-areas", get(service_areas))
        .route("/service-area/{slug}", get(service_area_single))
        .route("/service-areas/{slug}", get(service_area_single))
        .route("/industries", get(industries))
        .route("/portfolio", get(case_studies))
        .route("/portfolio/{slug}", get(case_study_single))
        .route("/case-studies", get(case_studies))
        .route("/case-studies/{slug}", get(case_study_single))
        .route("/insights", get(insights))
        .route("/insights/category/{slug}", get(insights_by_category))
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
        .route(
            "/request-quote",
            get(request_quote).post(submit_request_quote),
        )
}
