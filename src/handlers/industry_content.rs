use std::collections::HashMap;

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{IndustryCardView, IndustryEditorView, IndustryRecord},
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardIndustriesTemplate, DashboardIndustryCreateTemplate,
        DashboardIndustryEditTemplate, HomeTemplate, IndustriesTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct IndustryForm {
    pub title: String,
    pub slug: Option<String>,
    pub short_description: String,
    pub overview: String,
    pub challenge_focus: Option<String>,
    pub solution_focus: Option<String>,
    pub icon_class: Option<String>,
    pub sort_order: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub og_image_url: Option<String>,
    pub featured: Option<String>,
    pub published: Option<String>,
}

fn clean_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn build_editor_view(form: &IndustryForm) -> IndustryEditorView {
    IndustryEditorView {
        title: form.title.trim().to_string(),
        slug: form.slug.as_deref().unwrap_or_default().trim().to_string(),
        short_description: form.short_description.trim().to_string(),
        overview: form.overview.trim().to_string(),
        challenge_focus: form
            .challenge_focus
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        solution_focus: form
            .solution_focus
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        icon_class: form
            .icon_class
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

fn validate_industry_form(view: &IndustryEditorView) -> Result<(), &'static str> {
    if view.title.len() < 3 || view.short_description.len() < 20 || view.overview.len() < 60 {
        return Err(
            "Please provide a stronger title, short description, and overview for the industry entry.",
        );
    }

    if view.normalized_slug().len() < 3 {
        return Err("Please provide a valid slug or title.");
    }

    Ok(())
}

pub async fn fetch_home_featured_industries(state: &AppState) -> Vec<IndustryCardView> {
    match sqlx::query_as::<_, IndustryRecord>(
        r#"
        SELECT *
        FROM industries
        WHERE published = TRUE
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        LIMIT 8
        "#,
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(records) => records
            .into_iter()
            .map(|record| record.to_card_view())
            .collect(),
        Err(error) => {
            eprintln!("Failed to fetch featured industries: {error}");
            Vec::new()
        }
    }
}

async fn fetch_public_industries(state: &AppState) -> Result<Vec<IndustryRecord>, sqlx::Error> {
    sqlx::query_as::<_, IndustryRecord>(
        r#"
        SELECT *
        FROM industries
        WHERE published = TRUE
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_dashboard_industries(state: &AppState) -> Result<Vec<IndustryRecord>, sqlx::Error> {
    sqlx::query_as::<_, IndustryRecord>(
        r#"
        SELECT *
        FROM industries
        ORDER BY published DESC, featured DESC, sort_order ASC, updated_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_industry_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<IndustryRecord>, sqlx::Error> {
    sqlx::query_as::<_, IndustryRecord>(
        r#"
        SELECT *
        FROM industries
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let featured_portfolios = super::portfolio::fetch_home_featured_portfolios(&state).await;
    let featured_insights = super::insight_content::fetch_home_featured_insights(&state).await;
    let featured_industries = fetch_home_featured_industries(&state).await;

    render(HomeTemplate {
        featured_portfolios,
        featured_insights,
        featured_industries,
    })
}

pub async fn industries(State(state): State<AppState>) -> impl IntoResponse {
    match fetch_public_industries(&state).await {
        Ok(records) => {
            let industries: Vec<IndustryCardView> =
                records.iter().map(IndustryRecord::to_card_view).collect();
            let featured_count = records.iter().filter(|record| record.featured).count();

            render(IndustriesTemplate {
                industries,
                total_count: records.len(),
                featured_count,
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load industries: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load industries.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_industries(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_industries(&state).await {
        Ok(industries) => render(DashboardIndustriesTemplate {
            industries,
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load dashboard industries: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load dashboard industries.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_industry_create() -> impl IntoResponse {
    render(DashboardIndustryCreateTemplate {
        industry: IndustryEditorView::empty(),
        action_url: "/dashboard/industries".to_string(),
    })
}

pub async fn dashboard_industry_store(
    State(state): State<AppState>,
    Form(form): Form<IndustryForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_industry_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let published_at = if view.published {
        Some(Utc::now())
    } else {
        None
    };

    match sqlx::query(
        r#"
        INSERT INTO industries
        (
            title, slug, short_description, overview, challenge_focus, solution_focus,
            icon_class, featured, published, sort_order, meta_title, meta_description,
            canonical_url, og_image_url, published_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        "#,
    )
    .bind(&view.title)
    .bind(view.normalized_slug())
    .bind(&view.short_description)
    .bind(&view.overview)
    .bind(clean_optional(&form.challenge_focus))
    .bind(clean_optional(&form.solution_focus))
    .bind(clean_optional(&form.icon_class))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/industries?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to create industry: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create industry. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_industry_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_industry_by_id(&state, id).await {
        Ok(Some(industry)) => render(DashboardIndustryEditTemplate {
            industry: industry.to_editor_view(),
            action_url: format!("/dashboard/industries/{}/edit", industry.id),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Industry not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load industry for editing: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load industry.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_industry_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<IndustryForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_industry_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let existing = match fetch_industry_by_id(&state, id).await {
        Ok(Some(industry)) => industry,
        Ok(None) => return (StatusCode::NOT_FOUND, "Industry not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch industry before update: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update industry.",
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
        UPDATE industries
        SET
            title = $2,
            slug = $3,
            short_description = $4,
            overview = $5,
            challenge_focus = $6,
            solution_focus = $7,
            icon_class = $8,
            featured = $9,
            published = $10,
            sort_order = $11,
            meta_title = $12,
            meta_description = $13,
            canonical_url = $14,
            og_image_url = $15,
            published_at = $16,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(&view.title)
    .bind(view.normalized_slug())
    .bind(&view.short_description)
    .bind(&view.overview)
    .bind(clean_optional(&form.challenge_focus))
    .bind(clean_optional(&form.solution_focus))
    .bind(clean_optional(&form.icon_class))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/industries?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to update industry: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update industry. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_industry_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM industries
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/industries?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete industry: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete industry.",
            )
                .into_response()
        }
    }
}
