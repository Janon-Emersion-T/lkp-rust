use axum::{
    extract::{Form, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{ContactMessage, LeadFilters, LeadStats},
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardContactMessageShowTemplate, DashboardContactMessagesTemplate,
        DashboardLeadsTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct ContactMessageForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub service_interest: Option<String>,
    pub budget_range: Option<String>,
    pub project_timeline: Option<String>,
    pub subject: String,
    pub message: String,

    // Honeypot field. Real users will never fill this.
    // Bots usually fill every input they see.
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadQuery {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub service: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadUpdateForm {
    pub status: String,
    pub priority: String,
    pub lead_score: i32,
    pub assigned_to: Option<String>,
    pub internal_note: Option<String>,
    pub admin_reply: Option<String>,
    pub next_follow_up_at: Option<String>,
    pub lost_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuickStatusForm {
    pub status: String,
    pub redirect_to: Option<String>,
}

fn clean_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn calculate_lead_score(form: &ContactMessageForm) -> i32 {
    let mut score = 10;

    if clean_optional(&form.phone).is_some() {
        score += 15;
    }

    if clean_optional(&form.company).is_some() {
        score += 10;
    }

    if clean_optional(&form.service_interest).is_some() {
        score += 20;
    }

    if form.budget_range.as_deref().is_some_and(|value| {
        value.contains("500") || value.contains("1000") || value.contains("enterprise")
    }) {
        score += 20;
    }

    if form
        .project_timeline
        .as_deref()
        .is_some_and(|value| value.contains("urgent") || value.contains("this_month"))
    {
        score += 20;
    }

    if form.message.len() > 120 {
        score += 10;
    }

    score.clamp(0, 100)
}

fn calculate_priority(score: i32) -> &'static str {
    match score {
        70..=100 => "high",
        40..=69 => "medium",
        _ => "normal",
    }
}

fn parse_filters(query: LeadQuery) -> LeadFilters {
    LeadFilters {
        status: query.status.unwrap_or_else(|| "all".to_string()),
        priority: query.priority.unwrap_or_else(|| "all".to_string()),
        service: query.service.unwrap_or_else(|| "all".to_string()),
        q: query.q.unwrap_or_default(),
    }
}

fn parse_follow_up(value: Option<String>) -> Option<DateTime<Utc>> {
    let value = value?.trim().to_string();

    if value.is_empty() {
        return None;
    }

    DateTime::parse_from_rfc3339(&format!("{value}:00Z"))
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn current_status_timestamp_sql(status: &str) -> &'static str {
    match status {
        "contacted" => "contacted_at = COALESCE(contacted_at, NOW()),",
        "qualified" => "qualified_at = COALESCE(qualified_at, NOW()),",
        "converted" => "converted_at = COALESCE(converted_at, NOW()),",
        "archived" => "archived_at = COALESCE(archived_at, NOW()),",
        "spam" => "spam_at = COALESCE(spam_at, NOW()),",
        _ => "",
    }
}

fn client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(',').next().unwrap_or(value).trim().to_string())
}

fn user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

pub async fn submit_contact_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();
    let subject = form.subject.trim();
    let message = form.message.trim();

    // Silent bot trap.
    // If this hidden field has a value, act successful but do not store the lead.
    if clean_optional(&form.website).is_some() {
        return Redirect::to("/contact?success=1").into_response();
    }

    // Basic abuse protection.
    // Keep this simple for now. Later we can add rate limiting.
    if name.len() > 120 || email.len() > 180 || subject.len() > 180 || message.len() > 5000 {
        return (
            StatusCode::BAD_REQUEST,
            "Your message is too long. Please shorten it and try again.",
        )
            .into_response();
    }

    if name.len() < 2 || email.len() < 5 || subject.len() < 3 || message.len() < 10 {
        return (
            StatusCode::BAD_REQUEST,
            "Please complete the contact form correctly.",
        )
            .into_response();
    }

    let lead_score = calculate_lead_score(&form);
    let priority = calculate_priority(lead_score);

    let result = sqlx::query(
        r#"
        INSERT INTO contact_messages
        (
            name,
            email,
            phone,
            company,
            service_interest,
            budget_range,
            project_timeline,
            subject,
            message,
            source,
            status,
            priority,
            lead_score,
            client_ip,
            user_agent
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'contact_page', 'new', $10, $11, $12, $13)
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(clean_optional(&form.phone))
    .bind(clean_optional(&form.company))
    .bind(clean_optional(&form.service_interest))
    .bind(clean_optional(&form.budget_range))
    .bind(clean_optional(&form.project_timeline))
    .bind(subject)
    .bind(message)
    .bind(priority)
    .bind(lead_score)
    .bind(client_ip(&headers))
    .bind(user_agent(&headers))
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Redirect::to("/contact?success=1").into_response(),
        Err(error) => {
            eprintln!("Failed to submit contact message: {error}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit contact message.",
            )
                .into_response()
        }
    }
}

async fn fetch_stats(state: &AppState) -> LeadStats {
    sqlx::query_as::<_, LeadStats>(
        r#"
        SELECT
            COUNT(*) AS total,
            COUNT(*) FILTER (WHERE status = 'new') AS new_count,
            COUNT(*) FILTER (WHERE status = 'contacted') AS contacted_count,
            COUNT(*) FILTER (WHERE status = 'qualified') AS qualified_count,
            COUNT(*) FILTER (WHERE status = 'converted') AS converted_count,
            COUNT(*) FILTER (WHERE status = 'archived') AS archived_count,
            COUNT(*) FILTER (WHERE status = 'spam') AS spam_count,
            COUNT(*) FILTER (WHERE priority = 'high') AS high_priority_count,
            AVG(lead_score)::float8 AS avg_score
        FROM contact_messages
        "#,
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(LeadStats {
        total: 0,
        new_count: 0,
        contacted_count: 0,
        qualified_count: 0,
        converted_count: 0,
        archived_count: 0,
        spam_count: 0,
        high_priority_count: 0,
        avg_score: Some(0.0),
    })
}

async fn fetch_filtered_messages(state: &AppState, filters: &LeadFilters) -> Vec<ContactMessage> {
    let q = format!("%{}%", filters.q.trim());

    sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        WHERE
            ($1 = 'all' OR status = $1)
            AND ($2 = 'all' OR priority = $2)
            AND ($3 = 'all' OR service_interest = $3)
            AND (
                $4 = '%%'
                OR name ILIKE $4
                OR email ILIKE $4
                OR phone ILIKE $4
                OR company ILIKE $4
                OR subject ILIKE $4
                OR message ILIKE $4
            )
        ORDER BY
            CASE priority
                WHEN 'high' THEN 1
                WHEN 'medium' THEN 2
                ELSE 3
            END,
            lead_score DESC,
            created_at DESC
        LIMIT 150
        "#,
    )
    .bind(&filters.status)
    .bind(&filters.priority)
    .bind(&filters.service)
    .bind(q)
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|error| {
        eprintln!("Failed to fetch filtered contact messages: {error}");
        Vec::new()
    })
}

pub async fn dashboard_leads(
    State(state): State<AppState>,
    Query(query): Query<LeadQuery>,
) -> impl IntoResponse {
    let filters = parse_filters(query);
    let stats = fetch_stats(&state).await;
    let leads = fetch_filtered_messages(&state, &filters).await;

    render(DashboardLeadsTemplate {
        leads,
        stats,
        filters,
    })
}

pub async fn dashboard_contact_messages(
    State(state): State<AppState>,
    Query(query): Query<LeadQuery>,
) -> impl IntoResponse {
    let filters = parse_filters(query);
    let stats = fetch_stats(&state).await;
    let messages = fetch_filtered_messages(&state, &filters).await;

    render(DashboardContactMessagesTemplate {
        messages,
        stats,
        filters,
    })
}

pub async fn dashboard_contact_message_show(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let message = sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    match message {
        Ok(Some(message)) => {
            render(DashboardContactMessageShowTemplate { message }).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Contact message not found").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch contact message: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch contact message.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_contact_message_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<LeadUpdateForm>,
) -> impl IntoResponse {
    let status_marker = current_status_timestamp_sql(&form.status);
    let next_follow_up_at = parse_follow_up(form.next_follow_up_at);

    let query = format!(
        r#"
        UPDATE contact_messages
        SET
            status = $1,
            priority = $2,
            lead_score = $3,
            assigned_to = $4,
            internal_note = $5,
            admin_reply = $6,
            next_follow_up_at = $7,
            lost_reason = $8,
            {status_marker}
            updated_at = NOW()
        WHERE id = $9
        "#
    );

    let result = sqlx::query(&query)
        .bind(form.status)
        .bind(form.priority)
        .bind(form.lead_score.clamp(0, 100))
        .bind(clean_optional(&form.assigned_to))
        .bind(clean_optional(&form.internal_note))
        .bind(clean_optional(&form.admin_reply))
        .bind(next_follow_up_at)
        .bind(clean_optional(&form.lost_reason))
        .bind(id)
        .execute(&state.db)
        .await;

    if let Err(error) = result {
        eprintln!("Failed to update contact message: {error}");
    }

    Redirect::to(&format!("/dashboard/contact-messages/{id}"))
}

pub async fn dashboard_contact_message_quick_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<QuickStatusForm>,
) -> impl IntoResponse {
    let status_marker = current_status_timestamp_sql(&form.status);

    let query = format!(
        r#"
        UPDATE contact_messages
        SET
            status = $1,
            {status_marker}
            updated_at = NOW()
        WHERE id = $2
        "#
    );

    if let Err(error) = sqlx::query(&query)
        .bind(form.status)
        .bind(id)
        .execute(&state.db)
        .await
    {
        eprintln!("Failed to quick-update contact message: {error}");
    }

    let redirect_to = form
        .redirect_to
        .as_deref()
        .filter(|value| value.starts_with("/dashboard"))
        .unwrap_or("/dashboard/contact-messages");

    Redirect::to(redirect_to)
}

#[derive(Debug, Deserialize)]
pub struct ReplyForm {
    pub admin_reply: Option<String>,
}

pub async fn dashboard_contact_message_reply(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<ReplyForm>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        UPDATE contact_messages
        SET
            admin_reply = $1,
            replied_at = NOW(),
            status = CASE
                WHEN status = 'new' THEN 'contacted'
                ELSE status
            END,
            contacted_at = CASE
                WHEN contacted_at IS NULL THEN NOW()
                ELSE contacted_at
            END,
            updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(clean_optional(&form.admin_reply))
    .bind(id)
    .execute(&state.db)
    .await;

    if let Err(error) = result {
        eprintln!("Failed to save contact message reply: {error}");
    }

    Redirect::to(&format!("/dashboard/contact-messages/{id}"))
}

pub async fn dashboard_contact_message_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(error) = sqlx::query(
        r#"
        DELETE FROM contact_messages
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        eprintln!("Failed to delete contact message: {error}");
    }

    Redirect::to("/dashboard/contact-messages")
}
