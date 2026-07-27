use std::collections::{BTreeSet, HashMap};

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{
        CareerApplicationEditorView, CareerApplicationRecord, CareerCardView, CareerEditorView,
        CareerRecord,
    },
    state::AppState,
};

use super::{
    render::render,
    templates::{
        CareerApplicationShowTemplate, CareerApplyTemplate, CareerSingleTemplate, CareersTemplate,
        DashboardCareerApplicationsTemplate, DashboardCareerCreateTemplate,
        DashboardCareerEditTemplate, DashboardCareersTemplate, NotFoundTemplate, WhyWorkTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct CareerForm {
    pub title: String,
    pub slug: Option<String>,
    pub team: Option<String>,
    pub location: Option<String>,
    pub workplace_mode: Option<String>,
    pub employment_type: Option<String>,
    pub experience_level: Option<String>,
    pub salary_range: Option<String>,
    pub summary: String,
    pub description: String,
    pub responsibilities: Option<String>,
    pub requirements: Option<String>,
    pub benefits: Option<String>,
    pub application_email: Option<String>,
    pub apply_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub sort_order: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub og_image_url: Option<String>,
    pub featured: Option<String>,
    pub published: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CareerApplicationForm {
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub resume_url: Option<String>,
    pub cover_letter: String,
    pub experience_summary: Option<String>,
    pub availability: Option<String>,
    pub expected_salary: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CareerApplicationStatusForm {
    pub status: String,
    pub internal_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CareerApplyQuery {
    pub success: Option<String>,
}

fn clean_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn build_career_editor_view(form: &CareerForm) -> CareerEditorView {
    CareerEditorView {
        title: form.title.trim().to_string(),
        slug: form.slug.as_deref().unwrap_or_default().trim().to_string(),
        team: form.team.as_deref().unwrap_or_default().trim().to_string(),
        location: form
            .location
            .as_deref()
            .unwrap_or("Jaffna, Sri Lanka")
            .trim()
            .to_string(),
        workplace_mode: form
            .workplace_mode
            .as_deref()
            .unwrap_or("Hybrid")
            .trim()
            .to_string(),
        employment_type: form
            .employment_type
            .as_deref()
            .unwrap_or("Full-time")
            .trim()
            .to_string(),
        experience_level: form
            .experience_level
            .as_deref()
            .unwrap_or("Mid-level")
            .trim()
            .to_string(),
        salary_range: form
            .salary_range
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        summary: form.summary.trim().to_string(),
        description: form.description.trim().to_string(),
        responsibilities: form
            .responsibilities
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        requirements: form
            .requirements
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        benefits: form
            .benefits
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        application_email: form
            .application_email
            .as_deref()
            .unwrap_or("careers@lkprofessionals.com")
            .trim()
            .to_string(),
        apply_url: form
            .apply_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        cover_image_url: form
            .cover_image_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        sort_order: form.sort_order.unwrap_or_default(),
        meta_title: form
            .meta_title
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        meta_description: form
            .meta_description
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        canonical_url: form
            .canonical_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        og_image_url: form
            .og_image_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        featured: form.featured.is_some(),
        published: form.published.is_some(),
    }
}

fn build_application_editor_view(form: &CareerApplicationForm) -> CareerApplicationEditorView {
    CareerApplicationEditorView {
        full_name: form.full_name.trim().to_string(),
        email: form.email.trim().to_ascii_lowercase(),
        phone: form.phone.as_deref().unwrap_or_default().trim().to_string(),
        location: form
            .location
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        linkedin_url: form
            .linkedin_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        portfolio_url: form
            .portfolio_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        resume_url: form
            .resume_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        cover_letter: form.cover_letter.trim().to_string(),
        experience_summary: form
            .experience_summary
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        availability: form
            .availability
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        expected_salary: form
            .expected_salary
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn validate_career_form(view: &CareerEditorView) -> Result<(), &'static str> {
    if view.title.len() < 3 || view.summary.len() < 30 || view.description.len() < 120 {
        return Err("Please provide a stronger title, summary, and role description.");
    }

    if view.normalized_slug().len() < 3 {
        return Err("Please provide a valid slug or title.");
    }

    Ok(())
}

fn validate_application_form(view: &CareerApplicationEditorView) -> Result<(), &'static str> {
    if view.full_name.len() < 3 || !is_valid_email(&view.email) || view.cover_letter.len() < 60 {
        return Err(
            "Please provide your full name, a valid email, and a stronger application note.",
        );
    }

    Ok(())
}

async fn fetch_public_careers(state: &AppState) -> Result<Vec<CareerRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerRecord>(
        r#"
        SELECT *
        FROM careers
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_dashboard_careers(state: &AppState) -> Result<Vec<CareerRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerRecord>(
        r#"
        SELECT *
        FROM careers
        ORDER BY published DESC, featured DESC, sort_order ASC, updated_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_career_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<Option<CareerRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerRecord>(
        r#"
        SELECT *
        FROM careers
        WHERE slug = $1 AND published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_career_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<CareerRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerRecord>(
        r#"
        SELECT *
        FROM careers
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_career_applications(
    state: &AppState,
) -> Result<Vec<CareerApplicationRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerApplicationRecord>(
        r#"
        SELECT *
        FROM career_applications
        ORDER BY
            CASE status
                WHEN 'new' THEN 0
                WHEN 'reviewing' THEN 1
                WHEN 'shortlisted' THEN 2
                WHEN 'interview' THEN 3
                WHEN 'closed' THEN 4
                ELSE 5
            END,
            created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_application_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<CareerApplicationRecord>, sqlx::Error> {
    sqlx::query_as::<_, CareerApplicationRecord>(
        r#"
        SELECT *
        FROM career_applications
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_related_careers(
    state: &AppState,
    current_id: Uuid,
    team: &str,
) -> Vec<CareerCardView> {
    let mut records = sqlx::query_as::<_, CareerRecord>(
        r#"
        SELECT *
        FROM careers
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
          AND id <> $1
          AND team = $2
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        LIMIT 3
        "#,
    )
    .bind(current_id)
    .bind(team)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    if records.is_empty() {
        records = sqlx::query_as::<_, CareerRecord>(
            r#"
            SELECT *
            FROM careers
            WHERE published = TRUE
              AND (published_at IS NULL OR published_at <= NOW())
              AND id <> $1
            ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
            LIMIT 3
            "#,
        )
        .bind(current_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    }

    records
        .into_iter()
        .map(|record| record.to_card_view())
        .collect()
}

pub async fn careers(State(state): State<AppState>) -> impl IntoResponse {
    match fetch_public_careers(&state).await {
        Ok(records) => {
            let careers: Vec<CareerCardView> =
                records.iter().map(CareerRecord::to_card_view).collect();
            let featured_roles = records
                .iter()
                .filter(|record| record.featured)
                .take(3)
                .map(CareerRecord::to_card_view)
                .collect();
            let teams = records
                .iter()
                .map(|record| record.team_label().to_string())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let open_roles = records.len();

            render(CareersTemplate {
                careers,
                featured_roles,
                open_roles,
                teams,
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load careers: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load careers.").into_response()
        }
    }
}

pub async fn why_work() -> impl IntoResponse {
    render(WhyWorkTemplate)
}

pub async fn career_single(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match fetch_career_by_slug(&state, &slug).await {
        Ok(Some(record)) => {
            let career = record.to_detail_view();
            let related = fetch_related_careers(&state, record.id, record.team_label()).await;

            render(CareerSingleTemplate { career, related }).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, render(NotFoundTemplate)).into_response(),
        Err(error) => {
            eprintln!("Failed to load career: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load career.").into_response()
        }
    }
}

pub async fn career_apply(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<CareerApplyQuery>,
) -> impl IntoResponse {
    match fetch_career_by_slug(&state, &slug).await {
        Ok(Some(record)) => render(CareerApplyTemplate {
            career: record.to_detail_view(),
            form: CareerApplicationEditorView::empty(),
            success: query.success.as_deref() == Some("1"),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, render(NotFoundTemplate)).into_response(),
        Err(error) => {
            eprintln!("Failed to load application page: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load application page.",
            )
                .into_response()
        }
    }
}

pub async fn submit_career_application(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<CareerApplicationForm>,
) -> impl IntoResponse {
    let view = build_application_editor_view(&form);

    if let Err(message) = validate_application_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let career = match fetch_career_by_slug(&state, &slug).await {
        Ok(Some(record)) => record,
        Ok(None) => return (StatusCode::NOT_FOUND, "Career not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load career before application: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit application.",
            )
                .into_response();
        }
    };

    match sqlx::query(
        r#"
        INSERT INTO career_applications
        (
            career_id, role_title_snapshot, full_name, email, phone, location, linkedin_url,
            portfolio_url, resume_url, cover_letter, experience_summary, availability,
            expected_salary, source, status, created_at, updated_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 'Website Career Page', 'new', NOW(), NOW())
        "#,
    )
    .bind(career.id)
    .bind(&career.title)
    .bind(&view.full_name)
    .bind(&view.email)
    .bind(clean_optional(&Some(view.phone)))
    .bind(clean_optional(&Some(view.location)))
    .bind(clean_optional(&Some(view.linkedin_url)))
    .bind(clean_optional(&Some(view.portfolio_url)))
    .bind(clean_optional(&Some(view.resume_url)))
    .bind(&view.cover_letter)
    .bind(clean_optional(&Some(view.experience_summary)))
    .bind(clean_optional(&Some(view.availability)))
    .bind(clean_optional(&Some(view.expected_salary)))
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to(&format!("/careers/{slug}/apply?success=1")).into_response(),
        Err(error) => {
            eprintln!("Failed to save application: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit application.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_careers(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_careers(&state).await {
        Ok(careers) => render(DashboardCareersTemplate {
            careers,
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load careers dashboard: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load careers.").into_response()
        }
    }
}

pub async fn dashboard_career_create() -> impl IntoResponse {
    render(DashboardCareerCreateTemplate {
        career: CareerEditorView::empty(),
        action_url: "/dashboard/careers".to_string(),
    })
}

pub async fn dashboard_career_store(
    State(state): State<AppState>,
    Form(form): Form<CareerForm>,
) -> impl IntoResponse {
    let view = build_career_editor_view(&form);

    if let Err(message) = validate_career_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let published_at = if view.published {
        Some(Utc::now())
    } else {
        None
    };

    match sqlx::query(
        r#"
        INSERT INTO careers
        (
            title, slug, team, location, workplace_mode, employment_type, experience_level,
            salary_range, summary, description, responsibilities, requirements, benefits,
            application_email, apply_url, cover_image_url, featured, published, sort_order,
            meta_title, meta_description, canonical_url, og_image_url, published_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
        "#,
    )
    .bind(&view.title)
    .bind(view.normalized_slug())
    .bind(clean_optional(&Some(view.team)))
    .bind(&view.location)
    .bind(&view.workplace_mode)
    .bind(&view.employment_type)
    .bind(&view.experience_level)
    .bind(clean_optional(&Some(view.salary_range)))
    .bind(&view.summary)
    .bind(&view.description)
    .bind(clean_optional(&Some(view.responsibilities)))
    .bind(clean_optional(&Some(view.requirements)))
    .bind(clean_optional(&Some(view.benefits)))
    .bind(clean_optional(&Some(view.application_email)))
    .bind(clean_optional(&Some(view.apply_url)))
    .bind(clean_optional(&Some(view.cover_image_url)))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&Some(view.meta_title)))
    .bind(clean_optional(&Some(view.meta_description)))
    .bind(clean_optional(&Some(view.canonical_url)))
    .bind(clean_optional(&Some(view.og_image_url)))
    .bind(published_at)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/careers?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to create career: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create career. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_career_by_id(&state, id).await {
        Ok(Some(career)) => render(DashboardCareerEditTemplate {
            career: career.to_editor_view(),
            action_url: format!("/dashboard/careers/{id}/edit"),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Career not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load career for edit: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load career.").into_response()
        }
    }
}

pub async fn dashboard_career_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<CareerForm>,
) -> impl IntoResponse {
    let view = build_career_editor_view(&form);

    if let Err(message) = validate_career_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let existing = match fetch_career_by_id(&state, id).await {
        Ok(Some(career)) => career,
        Ok(None) => return (StatusCode::NOT_FOUND, "Career not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch career before update: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update career.",
            )
                .into_response();
        }
    };

    let published_at = if view.published {
        existing.published_at.or(Some(Utc::now()))
    } else {
        None
    };

    match sqlx::query(
        r#"
        UPDATE careers
        SET
            title = $2,
            slug = $3,
            team = $4,
            location = $5,
            workplace_mode = $6,
            employment_type = $7,
            experience_level = $8,
            salary_range = $9,
            summary = $10,
            description = $11,
            responsibilities = $12,
            requirements = $13,
            benefits = $14,
            application_email = $15,
            apply_url = $16,
            cover_image_url = $17,
            featured = $18,
            published = $19,
            sort_order = $20,
            meta_title = $21,
            meta_description = $22,
            canonical_url = $23,
            og_image_url = $24,
            published_at = $25,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(&view.title)
    .bind(view.normalized_slug())
    .bind(clean_optional(&Some(view.team)))
    .bind(&view.location)
    .bind(&view.workplace_mode)
    .bind(&view.employment_type)
    .bind(&view.experience_level)
    .bind(clean_optional(&Some(view.salary_range)))
    .bind(&view.summary)
    .bind(&view.description)
    .bind(clean_optional(&Some(view.responsibilities)))
    .bind(clean_optional(&Some(view.requirements)))
    .bind(clean_optional(&Some(view.benefits)))
    .bind(clean_optional(&Some(view.application_email)))
    .bind(clean_optional(&Some(view.apply_url)))
    .bind(clean_optional(&Some(view.cover_image_url)))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&Some(view.meta_title)))
    .bind(clean_optional(&Some(view.meta_description)))
    .bind(clean_optional(&Some(view.canonical_url)))
    .bind(clean_optional(&Some(view.og_image_url)))
    .bind(published_at)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/careers?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to update career: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update career.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM careers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/careers?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete career: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete career.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_applications(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_career_applications(&state).await {
        Ok(applications) => {
            let total_count = applications.len();
            let new_count = applications
                .iter()
                .filter(|item| item.status == "new")
                .count();
            let shortlisted_count = applications
                .iter()
                .filter(|item| item.status == "shortlisted" || item.status == "interview")
                .count();

            render(DashboardCareerApplicationsTemplate {
                applications,
                total_count,
                new_count,
                shortlisted_count,
                updated: query.get("updated").is_some_and(|value| value == "1"),
                deleted: query.get("deleted").is_some_and(|value| value == "1"),
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load applications: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load career applications.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_application_show(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_application_by_id(&state, id).await {
        Ok(Some(application)) => {
            render(CareerApplicationShowTemplate { application }).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Application not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load application: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load application.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_application_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<CareerApplicationStatusForm>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        UPDATE career_applications
        SET status = $2,
            internal_notes = $3,
            reviewed_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(form.status.trim().to_ascii_lowercase())
    .bind(clean_optional(&form.internal_notes))
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/career-applications?updated=1").into_response(),
        Err(error) => {
            eprintln!("Failed to update application: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update application.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_career_application_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM career_applications
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/career-applications?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete application: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete application.",
            )
                .into_response()
        }
    }
}
