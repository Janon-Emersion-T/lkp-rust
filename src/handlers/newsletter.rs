use std::collections::HashMap;

use axum::{
    Json,
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{
        NewsletterCampaignEditorView, NewsletterCampaignRecord, NewsletterSubscriberEditorView,
        NewsletterSubscriberRecord,
    },
    services::newsletter::{NewsletterCampaignInput, plain_text_to_email_html, queue_campaign},
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardNewsletterSubscriberEditTemplate, DashboardNewsletterSubscribersTemplate,
        DashboardNewslettersTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct NewsletterSubscribeForm {
    pub email: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewsletterSubscriberForm {
    pub email: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewsletterBulkImportForm {
    pub emails: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewsletterCampaignForm {
    pub title: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
}

#[derive(Debug, Serialize)]
struct NewsletterSubscribeResponse {
    success: bool,
    message: String,
}

fn clean_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn normalize_email(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn build_subscriber_editor_view(form: &NewsletterSubscriberForm) -> NewsletterSubscriberEditorView {
    NewsletterSubscriberEditorView {
        email: normalize_email(&form.email),
        source: form
            .source
            .as_deref()
            .unwrap_or("Website")
            .trim()
            .to_string(),
    }
}

fn build_campaign_editor_view(form: &NewsletterCampaignForm) -> NewsletterCampaignEditorView {
    NewsletterCampaignEditorView {
        title: form.title.trim().to_string(),
        subject: form.subject.trim().to_string(),
        preview_text: form
            .preview_text
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        content_html: form.content_html.trim().to_string(),
        cta_label: form
            .cta_label
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
        cta_url: form
            .cta_url
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

fn validate_subscriber(view: &NewsletterSubscriberEditorView) -> Result<(), &'static str> {
    if !is_valid_email(&view.email) {
        return Err("Please enter a valid email address.");
    }

    Ok(())
}

fn validate_campaign(view: &NewsletterCampaignEditorView) -> Result<(), &'static str> {
    if view.title.len() < 3 || view.subject.len() < 6 || view.content_html.len() < 40 {
        return Err("Please provide a stronger newsletter title, subject, and message.");
    }

    Ok(())
}

fn extract_emails(value: &str) -> Vec<String> {
    value
        .split(|character: char| matches!(character, ',' | ';' | '\n' | '\r' | '\t' | ' '))
        .map(normalize_email)
        .filter(|email| is_valid_email(email))
        .collect()
}

async fn fetch_dashboard_subscribers(
    state: &AppState,
) -> Result<Vec<NewsletterSubscriberRecord>, sqlx::Error> {
    sqlx::query_as::<_, NewsletterSubscriberRecord>(
        r#"
        SELECT *
        FROM newsletter_subscribers
        ORDER BY subscribed_at DESC, email ASC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

async fn fetch_subscriber_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Option<NewsletterSubscriberRecord>, sqlx::Error> {
    sqlx::query_as::<_, NewsletterSubscriberRecord>(
        r#"
        SELECT *
        FROM newsletter_subscribers
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
}

async fn fetch_newsletter_campaigns(
    state: &AppState,
) -> Result<Vec<NewsletterCampaignRecord>, sqlx::Error> {
    sqlx::query_as::<_, NewsletterCampaignRecord>(
        r#"
        SELECT *
        FROM newsletter_campaigns
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
}

pub async fn subscribe_newsletter(
    State(state): State<AppState>,
    Form(form): Form<NewsletterSubscribeForm>,
) -> impl IntoResponse {
    let email = normalize_email(&form.email);

    if !is_valid_email(&email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(NewsletterSubscribeResponse {
                success: false,
                message: "Enter a valid email address.".to_string(),
            }),
        );
    }

    match sqlx::query(
        r#"
        INSERT INTO newsletter_subscribers (email, source, subscribed_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        ON CONFLICT (email) DO NOTHING
        "#,
    )
    .bind(&email)
    .bind(clean_optional(&form.source).or_else(|| Some("Website Footer".to_string())))
    .execute(&state.db)
    .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(NewsletterSubscribeResponse {
                success: true,
                message: "You are subscribed. Future insights and updates will land in your inbox."
                    .to_string(),
            }),
        ),
        Err(error) => {
            eprintln!("Failed to subscribe newsletter user: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NewsletterSubscribeResponse {
                    success: false,
                    message: "Subscription failed. Please try again.".to_string(),
                }),
            )
        }
    }
}

pub async fn dashboard_newsletter_subscribers(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_dashboard_subscribers(&state).await {
        Ok(subscribers) => render(DashboardNewsletterSubscribersTemplate {
            subscribers,
            subscriber: NewsletterSubscriberEditorView::empty(),
            action_url: "/dashboard/newsletter-subscribers".to_string(),
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
            imported_count: query
                .get("imported")
                .and_then(|value| value.parse().ok())
                .unwrap_or(0),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load subscribers: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load newsletter subscribers.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_subscriber_store(
    State(state): State<AppState>,
    Form(form): Form<NewsletterSubscriberForm>,
) -> impl IntoResponse {
    let view = build_subscriber_editor_view(&form);

    if let Err(message) = validate_subscriber(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    match sqlx::query(
        r#"
        INSERT INTO newsletter_subscribers (email, source, subscribed_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        ON CONFLICT (email) DO UPDATE SET
            source = EXCLUDED.source,
            updated_at = NOW()
        "#,
    )
    .bind(&view.email)
    .bind(Some(view.source))
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/newsletter-subscribers?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to store subscriber: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save subscriber.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_subscriber_bulk_import(
    State(state): State<AppState>,
    Form(form): Form<NewsletterBulkImportForm>,
) -> impl IntoResponse {
    let emails = extract_emails(&form.emails);
    let source = clean_optional(&form.source).or_else(|| Some("Bulk Import".to_string()));

    if emails.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Add at least one valid email address for import.",
        )
            .into_response();
    }

    let mut imported_count = 0usize;

    for email in emails {
        match sqlx::query(
            r#"
            INSERT INTO newsletter_subscribers (email, source, subscribed_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            ON CONFLICT (email) DO NOTHING
            "#,
        )
        .bind(&email)
        .bind(source.clone())
        .execute(&state.db)
        .await
        {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    imported_count += 1;
                }
            }
            Err(error) => {
                eprintln!("Failed to import subscriber {email}: {error}");
            }
        }
    }

    Redirect::to(&format!(
        "/dashboard/newsletter-subscribers?imported={imported_count}"
    ))
    .into_response()
}

pub async fn dashboard_newsletter_subscriber_edit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match fetch_subscriber_by_id(&state, id).await {
        Ok(Some(subscriber)) => render(DashboardNewsletterSubscriberEditTemplate {
            subscriber: subscriber.to_editor_view(),
            action_url: format!("/dashboard/newsletter-subscribers/{id}/edit"),
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Subscriber not found.").into_response(),
        Err(error) => {
            eprintln!("Failed to load subscriber for editing: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load subscriber.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_subscriber_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<NewsletterSubscriberForm>,
) -> impl IntoResponse {
    let view = build_subscriber_editor_view(&form);

    if let Err(message) = validate_subscriber(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    match sqlx::query(
        r#"
        UPDATE newsletter_subscribers
        SET email = $2,
            source = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(&view.email)
    .bind(Some(view.source))
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/newsletter-subscribers?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to update subscriber: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update subscriber.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_subscriber_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM newsletter_subscribers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/newsletter-subscribers?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete subscriber: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete subscriber.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletters(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match fetch_newsletter_campaigns(&state).await {
        Ok(campaigns) => render(DashboardNewslettersTemplate {
            campaigns,
            campaign: NewsletterCampaignEditorView::empty(),
            action_url: "/dashboard/newsletters".to_string(),
            saved: query.get("saved").is_some_and(|value| value == "1"),
            deleted: query.get("deleted").is_some_and(|value| value == "1"),
        })
        .into_response(),
        Err(error) => {
            eprintln!("Failed to load newsletters: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to load newsletters.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_store(
    State(state): State<AppState>,
    Form(form): Form<NewsletterCampaignForm>,
) -> impl IntoResponse {
    let view = build_campaign_editor_view(&form);

    if let Err(message) = validate_campaign(&view) {
        return (StatusCode::BAD_REQUEST, message).into_response();
    }

    match queue_campaign(
        &state.db,
        NewsletterCampaignInput {
            title: view.title,
            subject: view.subject,
            preview_text: clean_optional(&Some(view.preview_text)),
            content_html: plain_text_to_email_html(&view.content_html),
            cta_label: clean_optional(&Some(view.cta_label)),
            cta_url: clean_optional(&Some(view.cta_url)),
            source_type: "manual".to_string(),
            source_id: None,
        },
    )
    .await
    {
        Ok(_) => Redirect::to("/dashboard/newsletters?saved=1").into_response(),
        Err(error) => {
            eprintln!("Failed to queue newsletter: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to queue newsletter.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_newsletter_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query(
        r#"
        DELETE FROM newsletter_campaigns
        WHERE id = $1
          AND source_type = 'manual'
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        Ok(_) => Redirect::to("/dashboard/newsletters?deleted=1").into_response(),
        Err(error) => {
            eprintln!("Failed to delete newsletter: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete newsletter.",
            )
                .into_response()
        }
    }
}
