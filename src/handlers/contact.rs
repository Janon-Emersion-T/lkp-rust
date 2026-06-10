use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{ContactMessage, LeadStats},
    state::AppState,
};

use super::{
    render::render,
    templates::{DashboardContactMessagesTemplate, DashboardLeadsTemplate},
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
}

#[derive(Debug, Deserialize)]
pub struct ReplyForm {
    pub admin_reply: String,
}

fn calculate_lead_score(form: &ContactMessageForm) -> i32 {
    let mut score = 10;

    if form.phone.as_deref().is_some_and(|v| !v.trim().is_empty()) {
        score += 15;
    }

    if form.company.as_deref().is_some_and(|v| !v.trim().is_empty()) {
        score += 10;
    }

    if form.service_interest.as_deref().is_some_and(|v| !v.trim().is_empty()) {
        score += 20;
    }

    if form.budget_range.as_deref().is_some_and(|v| {
        v.contains("500") || v.contains("1000") || v.contains("enterprise")
    }) {
        score += 20;
    }

    if form.project_timeline.as_deref().is_some_and(|v| {
        v.contains("urgent") || v.contains("this_month")
    }) {
        score += 20;
    }

    score
}

fn calculate_priority(score: i32) -> &'static str {
    match score {
        70..=100 => "high",
        40..=69 => "medium",
        _ => "normal",
    }
}

pub async fn submit_contact_message(
    State(state): State<AppState>,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();
    let subject = form.subject.trim();
    let message = form.message.trim();

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
            lead_score
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'contact_page', 'new', $10, $11)
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(form.phone.as_deref().map(str::trim).filter(|v| !v.is_empty()))
    .bind(form.company.as_deref().map(str::trim).filter(|v| !v.is_empty()))
    .bind(
        form.service_interest
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty()),
    )
    .bind(
        form.budget_range
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty()),
    )
    .bind(
        form.project_timeline
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty()),
    )
    .bind(subject)
    .bind(message)
    .bind(priority)
    .bind(lead_score)
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

pub async fn dashboard_leads(State(state): State<AppState>) -> impl IntoResponse {
    let leads = sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        ORDER BY
            CASE priority
                WHEN 'high' THEN 1
                WHEN 'medium' THEN 2
                ELSE 3
            END,
            created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|error| {
        eprintln!("Failed to fetch leads: {error}");
        Vec::new()
    });

    let stats = sqlx::query_as::<_, LeadStats>(
        r#"
        SELECT
            COUNT(*) AS total,
            COUNT(*) FILTER (WHERE status = 'new') AS new_count,
            COUNT(*) FILTER (WHERE status = 'contacted') AS contacted_count,
            COUNT(*) FILTER (WHERE status = 'qualified') AS qualified_count,
            COUNT(*) FILTER (WHERE status = 'converted') AS converted_count
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
    });

    render(DashboardLeadsTemplate { leads, stats })
}

pub async fn dashboard_contact_messages(State(state): State<AppState>) -> impl IntoResponse {
    let messages = sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|error| {
        eprintln!("Failed to fetch contact messages: {error}");
        Vec::new()
    });

    render(DashboardContactMessagesTemplate { messages })
}

pub async fn dashboard_contact_message_show(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        UPDATE contact_messages
        SET status = CASE WHEN status = 'new' THEN 'contacted' ELSE status END,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await;

    if let Err(error) = result {
        eprintln!("Failed to update contact message status: {error}");
    }

    Redirect::to("/dashboard/contact-messages")
}

pub async fn dashboard_contact_message_reply(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<ReplyForm>,
) -> impl IntoResponse {
    let reply = form.admin_reply.trim();

    if !reply.is_empty() {
        let result = sqlx::query(
            r#"
            UPDATE contact_messages
            SET admin_reply = $1,
                status = 'contacted',
                replied_at = NOW(),
                updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(reply)
        .bind(id)
        .execute(&state.db)
        .await;

        if let Err(error) = result {
            eprintln!("Failed to save admin reply: {error}");
        }
    }

    Redirect::to("/dashboard/contact-messages")
}