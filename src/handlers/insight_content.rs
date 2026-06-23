use std::collections::{BTreeMap, BTreeSet, HashMap};

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::TimeZone;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::shared::slugify,
    models::{InsightCardView, InsightEditorView, InsightRecord},
    services::newsletter::queue_insight_campaign,
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardInsightCategoryView, DashboardInsightCreateTemplate, DashboardInsightEditTemplate,
        DashboardInsightMetric, DashboardInsightTimelineView, DashboardInsightsTemplate,
        InsightCategoryLink, InsightSingleTemplate, InsightSnapshotMetric, InsightsTemplate,
        PaginationLink, PaginationView,
    },
};

const INSIGHTS_PER_PAGE: usize = 9;

#[derive(Debug, Deserialize)]
pub struct InsightForm {
    pub title: String,
    pub slug: Option<String>,
    pub excerpt: String,
    pub content: String,
    pub author: Option<String>,
    pub category: Option<String>,
    pub cover_image_url: Option<String>,
    pub published_at: Option<String>,
    pub reading_time_minutes: Option<i32>,
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

fn build_editor_view(form: &InsightForm) -> InsightEditorView {
    InsightEditorView {
        title: form.title.trim().to_string(),
        slug: form.slug.as_deref().unwrap_or_default().trim().to_string(),
        excerpt: form.excerpt.trim().to_string(),
        content: form.content.trim().to_string(),
        author: form
            .author
            .as_deref()
            .unwrap_or("LKProfessionals Team")
            .trim()
            .to_string(),
        category: form
            .category
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
        reading_time_minutes: form.reading_time_minutes.unwrap_or(5).max(1),
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
        published_at_local: form
            .published_at
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        featured: form.featured.is_some(),
        published: form.published.is_some(),
    }
}

fn validate_insight_form(view: &InsightEditorView) -> Result<(), &'static str> {
    if view.title.len() < 3 || view.excerpt.len() < 20 || view.content.len() < 120 {
        return Err("Please provide a stronger title, excerpt, and article body.");
    }

    if view.normalized_slug().len() < 3 {
        return Err("Please provide a valid slug or title.");
    }

    Ok(())
}

fn parse_optional_publish_at(
    value: &Option<String>,
) -> Result<Option<DateTime<Utc>>, &'static str> {
    let Some(raw_value) = value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };

    let naive = NaiveDateTime::parse_from_str(raw_value, "%Y-%m-%dT%H:%M")
        .map_err(|_| "Please provide a valid publish date and time.")?;
    let sri_lanka_offset =
        FixedOffset::east_opt(5 * 3600 + 30 * 60).ok_or("Failed to parse timezone.")?;
    let local_datetime = sri_lanka_offset
        .from_local_datetime(&naive)
        .single()
        .ok_or("Please provide a valid publish date and time.")?;

    Ok(Some(local_datetime.with_timezone(&Utc)))
}

fn resolve_publish_state(
    should_publish: bool,
    requested_publish_at: Option<DateTime<Utc>>,
    existing_publish_at: Option<DateTime<Utc>>,
) -> (bool, Option<DateTime<Utc>>) {
    if !should_publish {
        return (false, None);
    }

    let now = Utc::now();
    let publish_at = requested_publish_at.or(existing_publish_at).unwrap_or(now);

    (publish_at <= now, Some(publish_at))
}

pub async fn fetch_home_featured_insights(state: &AppState) -> Vec<InsightCardView> {
    match sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE published = TRUE
        ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC
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
            eprintln!("Failed to fetch featured insights: {error}");
            Vec::new()
        }
    }
}

async fn fetch_public_insights(state: &AppState) -> Result<Vec<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_public_insights_page(
    state: &AppState,
    limit: i64,
    offset: i64,
) -> Result<Vec<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
}

async fn fetch_public_insight_count(state: &AppState) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM insights
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        "#,
    )
    .fetch_one(&state.db)
    .await
}

async fn fetch_public_featured_insights(
    state: &AppState,
) -> Result<Vec<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
          AND featured = TRUE
        ORDER BY published_at DESC NULLS LAST, created_at DESC
        LIMIT 3
        "#,
    )
    .fetch_all(&state.db)
    .await
}

#[derive(Debug, Deserialize)]
pub struct InsightsQuery {
    pub page: Option<usize>,
}

#[derive(sqlx::FromRow)]
struct InsightStatsRecord {
    published_count: i64,
    total_reads: i64,
    average_read_time: f64,
}

async fn fetch_insight_stats(state: &AppState) -> Result<InsightStatsRecord, sqlx::Error> {
    sqlx::query_as::<_, InsightStatsRecord>(
        r#"
        SELECT
            COUNT(*)::BIGINT AS published_count,
            COALESCE(SUM(view_count), 0)::BIGINT AS total_reads,
            COALESCE(AVG(reading_time_minutes), 0)::FLOAT8 AS average_read_time
        FROM insights
        WHERE published = TRUE
        "#,
    )
    .fetch_one(&state.db)
    .await
}

async fn fetch_newsletter_subscriber_count(state: &AppState) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM newsletter_subscribers
        "#,
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0)
}

fn build_insights_page_url(page: usize) -> String {
    if page <= 1 {
        "/insights".to_string()
    } else {
        format!("/insights?page={page}")
    }
}

fn build_insight_category_page_url(category_slug: &str, page: usize) -> String {
    if page <= 1 {
        format!("/insights/category/{category_slug}")
    } else {
        format!("/insights/category/{category_slug}?page={page}")
    }
}

fn build_pagination_view(
    current_page: usize,
    total_pages: usize,
    page_url: impl Fn(usize) -> String,
) -> PaginationView {
    let previous_page_url = (current_page > 1).then(|| page_url(current_page - 1));
    let next_page_url = (current_page < total_pages).then(|| page_url(current_page + 1));

    let page_links = (1..=total_pages)
        .map(|page| PaginationLink {
            label: page.to_string(),
            url: page_url(page),
            active: page == current_page,
        })
        .collect();

    PaginationView {
        current_page,
        total_pages,
        previous_page_url,
        next_page_url,
        page_links,
    }
}

fn build_insight_category_links(
    all_records: &[InsightRecord],
    active_category: Option<&str>,
) -> Vec<InsightCategoryLink> {
    all_records
        .iter()
        .map(|record| record.category_label().to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|label| InsightCategoryLink {
            url: format!("/insights/category/{}", slugify(&label)),
            active: active_category.is_some_and(|current| current == label),
            label,
        })
        .collect()
}

fn build_insights_template(
    insights: Vec<InsightCardView>,
    featured_insights: Vec<InsightCardView>,
    categories: Vec<InsightCategoryLink>,
    snapshot_metrics: Vec<InsightSnapshotMetric>,
    pagination: PaginationView,
    active_category: Option<String>,
) -> InsightsTemplate {
    let category_name = active_category.clone();
    let archive_label = category_name
        .clone()
        .map(|label| format!("{label} Category"))
        .unwrap_or_else(|| "Insights & Articles".to_string());
    let archive_heading = category_name
        .clone()
        .map(|label| format!("{label} insights and articles"))
        .unwrap_or_else(|| {
            "Practical thinking on software, search, automation, and digital growth.".to_string()
        });
    let archive_description = category_name
        .clone()
        .map(|label| {
            format!(
                "Explore LKProfessionals articles in {label}, with practical guidance, strategy, and implementation insights."
            )
        })
        .unwrap_or_else(|| {
            "Browse dynamic articles from LKProfessionals covering web engineering, SEO strategy, AI-supported operations, and the systems that help modern businesses grow.".to_string()
        });
    let canonical_url = if let Some(label) = category_name.clone() {
        let slug = slugify(&label);
        if pagination.current_page > 1 {
            format!(
                "https://lkprofessionals.com/insights/category/{slug}?page={}",
                pagination.current_page
            )
        } else {
            format!("https://lkprofessionals.com/insights/category/{slug}")
        }
    } else if pagination.current_page > 1 {
        format!(
            "https://lkprofessionals.com/insights?page={}",
            pagination.current_page
        )
    } else {
        "https://lkprofessionals.com/insights".to_string()
    };
    let seo_title = category_name
        .clone()
        .map(|label| format!("{label} Insights & Articles | LKProfessionals"))
        .unwrap_or_else(|| "SEO, Software & Automation Insights | LKProfessionals".to_string());
    let meta_description = category_name
        .clone()
        .map(|label| {
            format!(
                "Read LKProfessionals articles in {label} covering strategy, implementation, and measurable digital growth."
            )
        })
        .unwrap_or_else(|| {
            "Read LKProfessionals insights on software, SEO, automation, and digital strategy to plan stronger websites, systems, campaigns, and business growth.".to_string()
        });
    let og_title = category_name
        .clone()
        .map(|label| format!("{label} Insights | LKProfessionals"))
        .unwrap_or_else(|| "SEO, Software & Automation Insights | LKProfessionals".to_string());
    let og_description = archive_description.clone();
    let twitter_title = og_title.clone();
    let twitter_description = meta_description.clone();
    let archive_schema_name = category_name
        .clone()
        .map(|label| format!("LKProfessionals {label} Insights"))
        .unwrap_or_else(|| "LKProfessionals Insights".to_string());

    InsightsTemplate {
        insights,
        featured_insights,
        categories,
        snapshot_metrics,
        pagination,
        seo_title,
        meta_description,
        canonical_url: canonical_url.clone(),
        og_title,
        og_description,
        og_url: canonical_url,
        twitter_title,
        twitter_description,
        archive_label,
        archive_heading,
        archive_description,
        archive_schema_name,
    }
}

async fn fetch_dashboard_insights(state: &AppState) -> Result<Vec<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        ORDER BY published DESC, featured DESC, updated_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_insight_by_slug(
    state: &AppState,
    slug: &str,
) -> Result<Option<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE slug = $1 AND published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_insight_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<InsightRecord>, sqlx::Error> {
    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_related_insights(
    state: &AppState,
    current_id: Uuid,
    category: Option<&str>,
) -> Vec<InsightCardView> {
    let related = if let Some(category) = category.filter(|value| !value.trim().is_empty()) {
        sqlx::query_as::<_, InsightRecord>(
            r#"
            SELECT *
            FROM insights
            WHERE published = TRUE
              AND (published_at IS NULL OR published_at <= NOW())
              AND id <> $1
              AND category = $2
            ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC
            LIMIT 3
            "#,
        )
        .bind(current_id)
        .bind(category)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
    } else {
        Vec::new()
    };

    if !related.is_empty() {
        return related
            .into_iter()
            .map(|record| record.to_card_view())
            .collect();
    }

    sqlx::query_as::<_, InsightRecord>(
        r#"
        SELECT *
        FROM insights
        WHERE published = TRUE
          AND (published_at IS NULL OR published_at <= NOW())
          AND id <> $1
        ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC
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

pub async fn insights(
    State(state): State<AppState>,
    Query(query): Query<InsightsQuery>,
) -> impl IntoResponse {
    let requested_page = query.page.unwrap_or(1).max(1);

    match fetch_public_insight_count(&state).await {
        Ok(total_count) => {
            let total_pages = ((total_count as usize).max(1)).div_ceil(INSIGHTS_PER_PAGE);
            let current_page = requested_page.min(total_pages.max(1));
            let offset = ((current_page - 1) * INSIGHTS_PER_PAGE) as i64;

            let page_records =
                match fetch_public_insights_page(&state, INSIGHTS_PER_PAGE as i64, offset).await {
                    Ok(records) => records,
                    Err(error) => {
                        eprintln!("Failed to load insights page: {error}");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to load insights.",
                        )
                            .into_response();
                    }
                };

            let all_records = fetch_public_insights(&state).await.unwrap_or_default();
            let featured_records = fetch_public_featured_insights(&state)
                .await
                .unwrap_or_default();

            let insights: Vec<InsightCardView> = page_records
                .iter()
                .map(InsightRecord::to_card_view)
                .collect();

            let featured_insights = featured_records
                .iter()
                .map(InsightRecord::to_card_view)
                .collect();

            let categories = build_insight_category_links(&all_records, None);

            let stats = fetch_insight_stats(&state)
                .await
                .unwrap_or(InsightStatsRecord {
                    published_count: total_count,
                    total_reads: all_records
                        .iter()
                        .map(|record| i64::from(record.view_count))
                        .sum(),
                    average_read_time: if all_records.is_empty() {
                        0.0
                    } else {
                        all_records
                            .iter()
                            .map(|record| f64::from(record.reading_time_minutes))
                            .sum::<f64>()
                            / all_records.len() as f64
                    },
                });
            let subscriber_count = fetch_newsletter_subscriber_count(&state).await;
            let snapshot_metrics = vec![
                InsightSnapshotMetric {
                    value: format_number(stats.published_count),
                    label: "Published Insights".to_string(),
                },
                InsightSnapshotMetric {
                    value: format_number(stats.total_reads),
                    label: "Total Reads".to_string(),
                },
                InsightSnapshotMetric {
                    value: format_number(subscriber_count),
                    label: "Subscribers".to_string(),
                },
                InsightSnapshotMetric {
                    value: format!("{}m", stats.average_read_time.round().max(1.0) as i64),
                    label: "Avg. Read Time".to_string(),
                },
            ];

            render(build_insights_template(
                insights,
                featured_insights,
                categories,
                snapshot_metrics,
                build_pagination_view(current_page, total_pages.max(1), build_insights_page_url),
                None,
            ))
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load insight count: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load insights.",
            )
                .into_response()
        }
    }
}

pub async fn insights_by_category(
    State(state): State<AppState>,
    Path(category_slug): Path<String>,
    Query(query): Query<InsightsQuery>,
) -> impl IntoResponse {
    let requested_page = query.page.unwrap_or(1).max(1);
    let all_records = match fetch_public_insights(&state).await {
        Ok(records) => records,
        Err(error) => {
            eprintln!("Failed to load insights for category page: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load insights.",
            )
                .into_response();
        }
    };

    let Some(active_category) = all_records
        .iter()
        .map(|record| record.category_label().to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .find(|label| slugify(label) == category_slug)
    else {
        return (StatusCode::NOT_FOUND, "Insight category not found.").into_response();
    };

    let filtered_records: Vec<InsightRecord> = all_records
        .iter()
        .filter(|record| record.category_label() == active_category)
        .cloned()
        .collect();

    let total_count = filtered_records.len();
    let total_pages = total_count.max(1).div_ceil(INSIGHTS_PER_PAGE);
    let current_page = requested_page.min(total_pages.max(1));
    let start = (current_page - 1) * INSIGHTS_PER_PAGE;
    let end = (start + INSIGHTS_PER_PAGE).min(total_count);
    let page_records = if start < end {
        filtered_records[start..end].to_vec()
    } else {
        Vec::new()
    };

    let featured_records: Vec<InsightRecord> = filtered_records
        .iter()
        .filter(|record| record.featured)
        .take(3)
        .cloned()
        .collect();
    let featured_records = if featured_records.is_empty() && current_page == 1 {
        filtered_records.iter().take(3).cloned().collect()
    } else {
        featured_records
    };

    let categories = build_insight_category_links(&all_records, Some(&active_category));
    let insights = page_records
        .iter()
        .map(InsightRecord::to_card_view)
        .collect::<Vec<_>>();
    let featured_insights = featured_records
        .iter()
        .map(InsightRecord::to_card_view)
        .collect::<Vec<_>>();
    let category_article_count = filtered_records.len();
    let category_total_reads = filtered_records
        .iter()
        .map(|record| i64::from(record.view_count))
        .sum::<i64>();
    let category_average_read_time = if filtered_records.is_empty() {
        0.0
    } else {
        filtered_records
            .iter()
            .map(|record| f64::from(record.reading_time_minutes))
            .sum::<f64>()
            / filtered_records.len() as f64
    };
    let latest_published = filtered_records
        .iter()
        .map(|record| record.published_at.unwrap_or(record.created_at))
        .max()
        .map(|value| value.format("%b %Y").to_string())
        .unwrap_or_else(|| "No updates".to_string());
    let snapshot_metrics = vec![
        InsightSnapshotMetric {
            value: format_number(category_article_count as i64),
            label: "Articles In Category".to_string(),
        },
        InsightSnapshotMetric {
            value: format_number(category_total_reads),
            label: "Category Reads".to_string(),
        },
        InsightSnapshotMetric {
            value: format!("{}m", category_average_read_time.round().max(1.0) as i64),
            label: "Avg. Read Time".to_string(),
        },
        InsightSnapshotMetric {
            value: latest_published,
            label: "Latest Update".to_string(),
        },
    ];

    render(build_insights_template(
        insights,
        featured_insights,
        categories,
        snapshot_metrics,
        build_pagination_view(current_page, total_pages.max(1), |page| {
            build_insight_category_page_url(&category_slug, page)
        }),
        Some(active_category),
    ))
    .into_response()
}

pub async fn insight_single(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match fetch_insight_by_slug(&state, &slug).await {
        Ok(Some(record)) => {
            if let Err(error) = sqlx::query(
                r#"
                UPDATE insights
                SET view_count = view_count + 1
                WHERE id = $1
                "#,
            )
            .bind(record.id)
            .execute(&state.db)
            .await
            {
                eprintln!("Failed to increment insight views: {error}");
            }

            let insight = record.to_detail_view();
            let related =
                fetch_related_insights(&state, record.id, record.category.as_deref()).await;

            render(InsightSingleTemplate { insight, related }).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Insight not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load insight: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load insight.").into_response()
        }
    }
}

pub async fn dashboard_insights(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_insights(&state).await {
        Ok(insights) => {
            let total_count = insights.len();
            let published_count = insights.iter().filter(|item| item.published).count();
            let scheduled_count = insights.iter().filter(|item| item.is_scheduled()).count();
            let draft_count = insights
                .iter()
                .filter(|item| !item.published && !item.is_scheduled())
                .count();
            let featured_count = insights.iter().filter(|item| item.featured).count();
            let total_views: i64 = insights.iter().map(|item| i64::from(item.view_count)).sum();
            let average_read_time = if insights.is_empty() {
                0.0
            } else {
                insights
                    .iter()
                    .map(|item| f64::from(item.reading_time_minutes))
                    .sum::<f64>()
                    / insights.len() as f64
            };

            let mut category_counts: BTreeMap<String, usize> = BTreeMap::new();
            let mut timeline_counts: BTreeMap<String, usize> = BTreeMap::new();

            for insight in &insights {
                *category_counts
                    .entry(insight.category_label().to_string())
                    .or_insert(0) += 1;

                let published = insight.published_at.unwrap_or(insight.created_at);
                *timeline_counts
                    .entry(published.format("%Y").to_string())
                    .or_insert(0) += 1;
            }

            let top_category = category_counts
                .iter()
                .max_by_key(|(_, count)| **count)
                .map(|(label, count)| format!("{label} ({count})"))
                .unwrap_or_else(|| "No categories yet".to_string());

            let latest_update = insights
                .iter()
                .map(|item| item.updated_at)
                .max()
                .map(|value| value.format("%d %b %Y").to_string())
                .unwrap_or_else(|| "No updates yet".to_string());

            let all_categories = category_counts.keys().cloned().collect::<Vec<_>>();

            let metrics = vec![
                DashboardInsightMetric {
                    label: "Total library".to_string(),
                    value: total_count.to_string(),
                    note: format!(
                        "{published_count} live, {scheduled_count} scheduled, {draft_count} drafts"
                    ),
                },
                DashboardInsightMetric {
                    label: "Featured pieces".to_string(),
                    value: featured_count.to_string(),
                    note: "Homepage and highlight-ready articles".to_string(),
                },
                DashboardInsightMetric {
                    label: "Total views".to_string(),
                    value: format_number(total_views),
                    note: "Combined public article page views".to_string(),
                },
                DashboardInsightMetric {
                    label: "Avg. read time".to_string(),
                    value: format!("{} min", average_read_time.round().max(1.0) as i64),
                    note: format!("Top category: {top_category} · Updated {latest_update}"),
                },
            ];

            let max_category_count = category_counts.values().copied().max().unwrap_or(1);
            let mut category_breakdown: Vec<DashboardInsightCategoryView> = category_counts
                .into_iter()
                .map(|(label, count)| DashboardInsightCategoryView {
                    label,
                    count,
                    width_percent: ((count * 100) / max_category_count).max(12),
                })
                .collect();
            category_breakdown.sort_by(|left, right| right.count.cmp(&left.count));
            category_breakdown.truncate(6);

            let max_timeline_count = timeline_counts.values().copied().max().unwrap_or(1);
            let mut timeline: Vec<DashboardInsightTimelineView> = timeline_counts
                .into_iter()
                .map(|(label, count)| DashboardInsightTimelineView {
                    label,
                    count,
                    height_percent: ((count * 100) / max_timeline_count).max(14),
                })
                .collect();
            timeline.sort_by(|left, right| left.label.cmp(&right.label));

            render(DashboardInsightsTemplate {
                insights,
                metrics,
                category_breakdown,
                all_categories,
                timeline,
                total_count,
                published_count,
                scheduled_count,
                draft_count,
                featured_count,
                saved: query.get("saved").is_some_and(|value| value == "1"),
                deleted: query.get("deleted").is_some_and(|value| value == "1"),
            })
            .into_response()
        }
        Err(error) => {
            eprintln!("Failed to load dashboard insights: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load dashboard insights.",
            )
                .into_response()
        }
    }
}

fn format_number(value: i64) -> String {
    let digits = value.max(0).to_string();
    let mut formatted = String::with_capacity(digits.len() + (digits.len() / 3));

    for (index, character) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(character);
    }

    formatted.chars().rev().collect()
}

pub async fn dashboard_insight_create() -> impl IntoResponse {
    render(DashboardInsightCreateTemplate {
        insight: InsightEditorView::empty(),
        action_url: "/dashboard/insights".to_string(),
    })
}

pub async fn dashboard_insight_store(
    State(state): State<AppState>,
    Form(form): Form<InsightForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_insight_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let requested_publish_at = match parse_optional_publish_at(&form.published_at) {
        Ok(value) => value,
        Err(message) => return (StatusCode::BAD_REQUEST, message).into_response(),
    };
    let (published, published_at) =
        resolve_publish_state(view.published, requested_publish_at, None);

    match sqlx::query(
        r#"
        INSERT INTO insights
        (
            title, slug, excerpt, content, author, category, cover_image_url,
            featured, published, reading_time_minutes, meta_title, meta_description,
            canonical_url, og_image_url, published_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING id, slug
        "#,
    )
    .bind(&view.title)
    .bind(view.normalized_slug())
    .bind(&view.excerpt)
    .bind(&view.content)
    .bind(&view.author)
    .bind(clean_optional(&form.category))
    .bind(clean_optional(&form.cover_image_url))
    .bind(view.featured)
    .bind(published)
    .bind(view.reading_time_minutes)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .fetch_one(&state.db)
    .await
    {
        Ok(row) => {
            if published {
                let id: Uuid = row.get("id");
                let slug: String = row.get("slug");

                if let Err(error) = queue_insight_campaign(
                    &state.db,
                    id,
                    &view.title,
                    &view.excerpt,
                    &format!("/insights/{slug}"),
                )
                .await
                {
                    eprintln!("Failed to queue insight campaign: {error}");
                }
            }

            Redirect::to("/dashboard/insights?saved=1").into_response()
        }
        Err(error) => {
            eprintln!("Failed to create insight: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create insight. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_insight_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_insight_by_id(&state, id).await {
        Ok(Some(insight)) => render(DashboardInsightEditTemplate {
            insight: insight.to_editor_view(),
            action_url: format!("/dashboard/insights/{}/edit", insight.id),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Insight not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load insight for editing: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load insight.").into_response()
        }
    }
}

pub async fn dashboard_insight_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<InsightForm>,
) -> impl IntoResponse {
    let view = build_editor_view(&form);

    if let Err(message) = validate_insight_form(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    let requested_publish_at = match parse_optional_publish_at(&form.published_at) {
        Ok(value) => value,
        Err(message) => return (StatusCode::BAD_REQUEST, message).into_response(),
    };

    let existing = match fetch_insight_by_id(&state, id).await {
        Ok(Some(insight)) => insight,
        Ok(None) => return (StatusCode::NOT_FOUND, "Insight not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch insight before update: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update insight.",
            )
                .into_response();
        }
    };

    let (published, published_at) =
        resolve_publish_state(view.published, requested_publish_at, existing.published_at);
    let was_live = existing.published
        && existing
            .published_at
            .map(|value| value <= Utc::now())
            .unwrap_or(true);

    match sqlx::query(
        r#"
        UPDATE insights
        SET
            title = $2,
            slug = $3,
            excerpt = $4,
            content = $5,
            author = $6,
            category = $7,
            cover_image_url = $8,
            featured = $9,
            published = $10,
            reading_time_minutes = $11,
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
    .bind(&view.excerpt)
    .bind(&view.content)
    .bind(&view.author)
    .bind(clean_optional(&form.category))
    .bind(clean_optional(&form.cover_image_url))
    .bind(view.featured)
    .bind(published)
    .bind(view.reading_time_minutes)
    .bind(clean_optional(&form.meta_title))
    .bind(clean_optional(&form.meta_description))
    .bind(clean_optional(&form.canonical_url))
    .bind(clean_optional(&form.og_image_url))
    .bind(published_at)
    .execute(&state.db)
    .await
    {
        Ok(_) => {
            if published && !was_live {
                if let Err(error) = queue_insight_campaign(
                    &state.db,
                    id,
                    &view.title,
                    &view.excerpt,
                    &format!("/insights/{}", view.normalized_slug()),
                )
                .await
                {
                    eprintln!("Failed to queue insight campaign: {error}");
                }
            }

            Redirect::to("/dashboard/insights?saved=1").into_response()
        }
        Err(error) => {
            eprintln!("Failed to update insight: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update insight. Make sure the slug is unique.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_insight_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM insights
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/insights?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete insight: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete insight.",
            )
                .into_response()
        }
    }
}
