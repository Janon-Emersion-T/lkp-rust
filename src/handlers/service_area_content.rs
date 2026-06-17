use std::collections::{BTreeMap, HashMap};

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    handlers::service_content::{ServiceCard, all_service_cards},
    models::{ServiceAreaEditorView, ServiceAreaRecord},
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardServiceAreaCreateTemplate, DashboardServiceAreaEditTemplate,
        DashboardServiceAreasTemplate, ServiceAreaCardView, ServiceAreaDetailTemplate,
        ServiceAreaGroupView, ServiceAreasTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct ServiceAreaForm {
    pub area_name: String,
    pub slug: Option<String>,
    pub area_type: String,
    pub country: String,
    pub market_region: String,
    pub short_description: String,
    pub overview: String,
    pub buyer_profile: Option<String>,
    pub delivery_focus: Option<String>,
    pub timezone_note: Option<String>,
    pub nearby_markets: Option<String>,
    pub hero_image_url: Option<String>,
    pub gallery_image_url_2: Option<String>,
    pub gallery_image_url_3: Option<String>,
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

fn build_editor_view(form: &ServiceAreaForm) -> ServiceAreaEditorView {
    ServiceAreaEditorView {
        area_name: form.area_name.trim().to_string(),
        slug: form.slug.as_deref().unwrap_or_default().trim().to_string(),
        area_type: form.area_type.trim().to_ascii_lowercase(),
        country: form.country.trim().to_string(),
        market_region: form.market_region.trim().to_string(),
        short_description: form.short_description.trim().to_string(),
        overview: form.overview.trim().to_string(),
        buyer_profile: form
            .buyer_profile
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        delivery_focus: form
            .delivery_focus
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        timezone_note: form
            .timezone_note
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        nearby_markets: form
            .nearby_markets
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        hero_image_url: form
            .hero_image_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        gallery_image_url_2: form
            .gallery_image_url_2
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        gallery_image_url_3: form
            .gallery_image_url_3
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

fn validate_service_area_form(view: &ServiceAreaEditorView) -> Result<(), &'static str> {
    if view.area_name.len() < 2
        || view.country.len() < 2
        || view.short_description.len() < 24
        || view.overview.len() < 80
    {
        return Err("Please provide stronger area, country, summary, and overview content.");
    }

    if !matches!(view.area_type.as_str(), "city" | "state" | "country") {
        return Err("Area type must be city, state, or country.");
    }

    if view.market_region.len() < 2 {
        return Err("Please provide a market region.");
    }

    if view.normalized_slug().len() < 2 {
        return Err("Please provide a valid slug or area name.");
    }

    Ok(())
}

pub async fn fetch_public_service_areas(
    state: &AppState,
) -> Result<Vec<ServiceAreaRecord>, sqlx::Error> {
    sqlx::query_as::<_, ServiceAreaRecord>(
        r#"
        SELECT *
        FROM service_areas
        WHERE published = TRUE
        ORDER BY market_region ASC, featured DESC, sort_order ASC, area_name ASC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_dashboard_service_areas(
    state: &AppState,
) -> Result<Vec<ServiceAreaRecord>, sqlx::Error> {
    sqlx::query_as::<_, ServiceAreaRecord>(
        r#"
        SELECT *
        FROM service_areas
        ORDER BY published DESC, featured DESC, market_region ASC, sort_order ASC, updated_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

pub async fn fetch_service_area_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<Option<ServiceAreaRecord>, sqlx::Error> {
    sqlx::query_as::<_, ServiceAreaRecord>(
        r#"
        SELECT *
        FROM service_areas
        WHERE slug = $1 AND published = TRUE
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_service_area_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<ServiceAreaRecord>, sqlx::Error> {
    sqlx::query_as::<_, ServiceAreaRecord>(
        r#"
        SELECT *
        FROM service_areas
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

pub async fn service_areas(State(state): State<AppState>) -> impl IntoResponse {
    match fetch_public_service_areas(&state).await {
        Ok(records) => {
            let mut grouped = BTreeMap::<String, Vec<ServiceAreaCardView>>::new();
            for record in &records {
                grouped
                    .entry(record.market_region.clone())
                    .or_default()
                    .push(record.to_card_view());
            }

            let groups = grouped
                .into_iter()
                .map(|(title, areas)| ServiceAreaGroupView {
                    description: group_description(&title).to_string(),
                    title,
                    areas,
                })
                .collect::<Vec<_>>();

            render(ServiceAreasTemplate {
                groups,
                all_areas: records
                    .iter()
                    .map(ServiceAreaRecord::to_card_view)
                    .collect(),
                total_areas: records.len(),
                featured_services: featured_service_area_services(),
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load service areas: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load service areas.",
            )
                .into_response()
        }
    }
}

pub async fn service_area_single(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match fetch_service_area_by_slug(&state, &slug).await {
        Ok(Some(record)) => {
            let related_areas = fetch_public_service_areas(&state)
                .await
                .unwrap_or_default()
                .into_iter()
                .filter(|area| area.slug != record.slug)
                .filter(|area| {
                    area.market_region == record.market_region || area.country == record.country
                })
                .take(4)
                .map(|area| area.to_card_view())
                .collect::<Vec<_>>();

            render(ServiceAreaDetailTemplate {
                page: record.to_page_view(),
                related_areas,
                featured_services: featured_service_area_services(),
            })
            .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Service area not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load service area: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load service area.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_service_areas(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_service_areas(&state).await {
        Ok(service_areas) => render(DashboardServiceAreasTemplate {
            service_areas,
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load dashboard service areas: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load service areas.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_service_area_create() -> impl IntoResponse {
    render(DashboardServiceAreaCreateTemplate {
        service_area: ServiceAreaEditorView::empty(),
        action_url: "/dashboard/service-areas".to_string(),
    })
}

pub async fn dashboard_service_area_store(
    State(state): State<AppState>,
    Form(form): Form<ServiceAreaForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_service_area_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let published_at = if view.published {
        Some(Utc::now())
    } else {
        None
    };

    match sqlx::query(
        r#"
        INSERT INTO service_areas
        (
            area_name, slug, area_type, country, market_region, short_description, overview,
            buyer_profile, delivery_focus, timezone_note, nearby_markets, hero_image_url,
            gallery_image_url_2, gallery_image_url_3, featured, published, sort_order,
            meta_title, meta_description, canonical_url, og_image_url, published_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
        "#,
    )
    .bind(&view.area_name)
    .bind(view.normalized_slug())
    .bind(&view.area_type)
    .bind(&view.country)
    .bind(&view.market_region)
    .bind(&view.short_description)
    .bind(&view.overview)
    .bind(clean_optional(&form.buyer_profile))
    .bind(clean_optional(&form.delivery_focus))
    .bind(clean_optional(&form.timezone_note))
    .bind(clean_optional(&form.nearby_markets))
    .bind(clean_optional(&form.hero_image_url))
    .bind(clean_optional(&form.gallery_image_url_2))
    .bind(clean_optional(&form.gallery_image_url_3))
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
        Ok(_) => Redirect::to("/dashboard/service-areas?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to create service area: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create service area. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_service_area_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_service_area_by_id(&state, id).await {
        Ok(Some(service_area)) => render(DashboardServiceAreaEditTemplate {
            service_area: service_area.to_editor_view(),
            action_url: format!("/dashboard/service-areas/{}/edit", service_area.id),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Service area not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load service area for editing: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load service area.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_service_area_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<ServiceAreaForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_service_area_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let existing = match fetch_service_area_by_id(&state, id).await {
        Ok(Some(service_area)) => service_area,
        Ok(None) => return (StatusCode::NOT_FOUND, "Service area not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch service area before update: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update service area.",
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
        UPDATE service_areas
        SET
            area_name = $2,
            slug = $3,
            area_type = $4,
            country = $5,
            market_region = $6,
            short_description = $7,
            overview = $8,
            buyer_profile = $9,
            delivery_focus = $10,
            timezone_note = $11,
            nearby_markets = $12,
            hero_image_url = $13,
            gallery_image_url_2 = $14,
            gallery_image_url_3 = $15,
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
    .bind(&view.area_name)
    .bind(view.normalized_slug())
    .bind(&view.area_type)
    .bind(&view.country)
    .bind(&view.market_region)
    .bind(&view.short_description)
    .bind(&view.overview)
    .bind(clean_optional(&form.buyer_profile))
    .bind(clean_optional(&form.delivery_focus))
    .bind(clean_optional(&form.timezone_note))
    .bind(clean_optional(&form.nearby_markets))
    .bind(clean_optional(&form.hero_image_url))
    .bind(clean_optional(&form.gallery_image_url_2))
    .bind(clean_optional(&form.gallery_image_url_3))
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
        Ok(_) => Redirect::to("/dashboard/service-areas?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to update service area: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update service area. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_service_area_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM service_areas
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/service-areas?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete service area: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete service area.",
            )
                .into_response()
        }
    }
}

pub fn featured_service_area_services() -> Vec<ServiceCard> {
    all_service_cards()
        .into_iter()
        .filter(|service| {
            matches!(
                service.slug,
                "/services/custom-software-development"
                    | "/services/web-development"
                    | "/services/seo-search-growth"
                    | "/services/ai-automation-solutions"
            )
        })
        .collect()
}

fn group_description(region: &str) -> &'static str {
    match region {
        "Sri Lanka" => {
            "Core local markets where LKProfessionals combines Jaffna-rooted delivery with national commercial reach."
        }
        "UK & Europe" => {
            "Markets looking for offshore software, web, SEO, and automation support with structured communication."
        }
        "India & South Asia" => {
            "Regional software and growth markets where practical execution and strong cost-to-capability matter."
        }
        "North America" => {
            "Cities where companies often compare local agencies against offshore and hybrid delivery partners."
        }
        "Middle East" => {
            "Commercial hubs where fast-moving teams need dependable remote engineering and digital execution."
        }
        "Asia-Pacific" => {
            "High-standard international markets where LKProfessionals can support software, web, and SEO delivery."
        }
        _ => "International delivery markets supported by LKProfessionals.",
    }
}
