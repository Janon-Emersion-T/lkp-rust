use std::collections::HashMap;

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::{PortfolioCardView, PortfolioEditorView, PortfolioRecord},
    services::newsletter::queue_portfolio_campaign,
    state::AppState,
};

use super::{
    render::render,
    templates::{
        CaseStudiesTemplate, DashboardPortfolioCreateTemplate, DashboardPortfolioEditTemplate,
        DashboardPortfoliosTemplate, PortfolioSingleTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct PortfolioForm {
    pub title: String,
    pub slug: Option<String>,
    pub client_name: Option<String>,
    pub industry: Option<String>,
    pub service_category: Option<String>,
    pub excerpt: String,
    pub overview: String,
    pub challenge: Option<String>,
    pub solution: Option<String>,
    pub results: Option<String>,
    pub impact_metrics: Option<String>,
    pub technologies: Option<String>,
    pub cover_image_url: Option<String>,
    pub live_url: Option<String>,
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

fn build_editor_view(form: &PortfolioForm) -> PortfolioEditorView {
    PortfolioEditorView {
        title: form.title.trim().to_string(),
        slug: form.slug.as_deref().unwrap_or_default().trim().to_string(),
        client_name: form
            .client_name
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        industry: form
            .industry
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        service_category: form
            .service_category
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        excerpt: form.excerpt.trim().to_string(),
        overview: form.overview.trim().to_string(),
        challenge: form
            .challenge
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        solution: form
            .solution
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        results: form
            .results
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        impact_metrics: form
            .impact_metrics
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        technologies: form
            .technologies
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
        live_url: form
            .live_url
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

fn validate_portfolio_form(view: &PortfolioEditorView) -> Result<(), &'static str> {
    if view.title.len() < 3 || view.excerpt.len() < 20 || view.overview.len() < 60 {
        return Err("Please provide a stronger title, excerpt, and overview for the case study.");
    }

    if view.normalized_slug().len() < 3 {
        return Err("Please provide a valid slug or title.");
    }

    Ok(())
}

pub(crate) async fn fetch_home_featured_portfolios(state: &AppState) -> Vec<PortfolioCardView> {
    match sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        WHERE published = TRUE
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        LIMIT 3
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
            eprintln!("Failed to fetch featured portfolios: {error}");
            Vec::new()
        }
    }
}

async fn fetch_public_portfolios(state: &AppState) -> Result<Vec<PortfolioRecord>, sqlx::Error> {
    sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        WHERE published = TRUE
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_dashboard_portfolios(state: &AppState) -> Result<Vec<PortfolioRecord>, sqlx::Error> {
    sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        ORDER BY published DESC, featured DESC, sort_order ASC, updated_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_portfolio_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<Option<PortfolioRecord>, sqlx::Error> {
    sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        WHERE slug = $1 AND published = TRUE
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_portfolio_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<PortfolioRecord>, sqlx::Error> {
    sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_related_portfolios(
    state: &AppState,
    current_id: Uuid,
    industry: Option<&str>,
) -> Vec<PortfolioCardView> {
    let records = if let Some(industry) = industry.filter(|value| !value.trim().is_empty()) {
        sqlx::query_as::<_, PortfolioRecord>(
            r#"
            SELECT *
            FROM portfolios
            WHERE published = TRUE
              AND id <> $1
              AND industry = $2
            ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
            LIMIT 3
            "#,
        )
        .bind(current_id)
        .bind(industry)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else {
        Vec::new()
    };

    if !records.is_empty() {
        return records
            .into_iter()
            .map(|record| record.to_card_view())
            .collect();
    }

    sqlx::query_as::<_, PortfolioRecord>(
        r#"
        SELECT *
        FROM portfolios
        WHERE published = TRUE
          AND id <> $1
        ORDER BY featured DESC, sort_order ASC, published_at DESC NULLS LAST, created_at DESC
        LIMIT 3
        "#,
    )
    .bind(current_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|record| record.to_card_view())
    .collect()
}

pub async fn case_studies(State(state): State<AppState>) -> impl IntoResponse {
    match fetch_public_portfolios(&state).await {
        Ok(records) => {
            let portfolio_cards: Vec<PortfolioCardView> =
                records.iter().map(PortfolioRecord::to_card_view).collect();

            let featured_portfolios = records
                .iter()
                .filter(|record| record.featured)
                .take(3)
                .map(PortfolioRecord::to_card_view)
                .collect();

            let featured_count = records.iter().filter(|record| record.featured).count();
            let industry_count = records
                .iter()
                .filter_map(|record| record.industry.as_deref())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .collect::<std::collections::BTreeSet<_>>()
                .len();

            render(CaseStudiesTemplate {
                portfolios: portfolio_cards,
                featured_portfolios,
                total_count: records.len(),
                featured_count,
                industry_count,
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load case studies: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load case studies.",
            )
                .into_response()
        }
    }
}

pub async fn case_study_single(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match fetch_portfolio_by_slug(&state, &slug).await {
        Ok(Some(record)) => {
            let detail = record.to_detail_view();
            let related =
                fetch_related_portfolios(&state, record.id, record.industry.as_deref()).await;

            render(PortfolioSingleTemplate {
                portfolio: detail,
                related,
            })
            .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Case study not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load case study: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load case study.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_portfolios(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_portfolios(&state).await {
        Ok(portfolios) => render(DashboardPortfoliosTemplate {
            portfolios,
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load dashboard portfolios: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load dashboard portfolios.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_portfolio_create() -> impl IntoResponse {
    render(DashboardPortfolioCreateTemplate {
        portfolio: PortfolioEditorView::empty(),
        action_url: "/dashboard/portfolios".to_string(),
    })
}

pub async fn dashboard_portfolio_store(
    State(state): State<AppState>,
    Form(form): Form<PortfolioForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_portfolio_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let slug = view.normalized_slug();
    let published_at = if view.published {
        Some(Utc::now())
    } else {
        None
    };

    let result = sqlx::query(
        r#"
        INSERT INTO portfolios
        (
            title,
            slug,
            client_name,
            industry,
            service_category,
            excerpt,
            overview,
            challenge,
            solution,
            results,
            impact_metrics,
            technologies,
            cover_image_url,
            live_url,
            featured,
            published,
            sort_order,
            meta_title,
            meta_description,
            canonical_url,
            og_image_url,
            published_at
        )
        VALUES
        (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22
        )
        RETURNING id, slug
        "#,
    )
    .bind(&view.title)
    .bind(slug)
    .bind(clean_optional(&form.client_name))
    .bind(clean_optional(&form.industry))
    .bind(clean_optional(&form.service_category))
    .bind(&view.excerpt)
    .bind(&view.overview)
    .bind(clean_optional(&form.challenge))
    .bind(clean_optional(&form.solution))
    .bind(clean_optional(&form.results))
    .bind(clean_optional(&form.impact_metrics))
    .bind(clean_optional(&form.technologies))
    .bind(clean_optional(&form.cover_image_url))
    .bind(clean_optional(&form.live_url))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            if view.published {
                let id: Uuid = row.get("id");
                let slug: String = row.get("slug");

                if let Err(error) = queue_portfolio_campaign(
                    &state.db,
                    id,
                    &view.title,
                    &view.excerpt,
                    &format!("/portfolio/{slug}"),
                )
                .await
                {
                    eprintln!("Failed to queue portfolio campaign: {error}");
                }
            }

            Redirect::to("/dashboard/portfolios?saved=1").into_response()
        }
        Err(error) => {
            eprintln!("Failed to create portfolio: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create case study. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_portfolio_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_portfolio_by_id(&state, id).await {
        Ok(Some(portfolio)) => render(DashboardPortfolioEditTemplate {
            portfolio: portfolio.to_editor_view(),
            action_url: format!("/dashboard/portfolios/{}/edit", portfolio.id),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Portfolio not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load portfolio for editing: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load case study.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_portfolio_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<PortfolioForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_portfolio_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let existing = match fetch_portfolio_by_id(&state, id).await {
        Ok(Some(portfolio)) => portfolio,
        Ok(None) => return (StatusCode::NOT_FOUND, "Portfolio not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch portfolio before update: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update case study.",
            )
                .into_response();
        }
    };

    let slug = view.normalized_slug();
    let published_at = if view.published {
        existing.published_at.or(Some(Utc::now()))
    } else {
        None
    };

    let result = sqlx::query(
        r#"
        UPDATE portfolios
        SET
            title = $2,
            slug = $3,
            client_name = $4,
            industry = $5,
            service_category = $6,
            excerpt = $7,
            overview = $8,
            challenge = $9,
            solution = $10,
            results = $11,
            impact_metrics = $12,
            technologies = $13,
            cover_image_url = $14,
            live_url = $15,
            featured = $16,
            published = $17,
            sort_order = $18,
            meta_title = $19,
            meta_description = $20,
            canonical_url = $21,
            og_image_url = $22,
            published_at = $23,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(&view.title)
    .bind(slug)
    .bind(clean_optional(&form.client_name))
    .bind(clean_optional(&form.industry))
    .bind(clean_optional(&form.service_category))
    .bind(&view.excerpt)
    .bind(&view.overview)
    .bind(clean_optional(&form.challenge))
    .bind(clean_optional(&form.solution))
    .bind(clean_optional(&form.results))
    .bind(clean_optional(&form.impact_metrics))
    .bind(clean_optional(&form.technologies))
    .bind(clean_optional(&form.cover_image_url))
    .bind(clean_optional(&form.live_url))
    .bind(view.featured)
    .bind(view.published)
    .bind(view.sort_order)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            if view.published && !existing.published {
                if let Err(error) = queue_portfolio_campaign(
                    &state.db,
                    id,
                    &view.title,
                    &view.excerpt,
                    &format!("/portfolio/{}", view.normalized_slug()),
                )
                .await
                {
                    eprintln!("Failed to queue portfolio campaign: {error}");
                }
            }

            Redirect::to("/dashboard/portfolios?saved=1").into_response()
        }
        Err(error) => {
            eprintln!("Failed to update portfolio: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update case study. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_portfolio_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM portfolios
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/portfolios?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete portfolio: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete case study.",
            )
                .into_response()
        }
    }
}
