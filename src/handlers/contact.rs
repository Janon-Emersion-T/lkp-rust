use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

use super::{render::render, templates::DashboardContactMessagesTemplate};

#[derive(Deserialize)]
pub struct ContactMessageForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub subject: String,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct ContactMessage {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub subject: String,
    pub message: String,
    pub status: String,
    pub admin_reply: Option<String>,
    pub replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ReplyForm {
    pub admin_reply: String,
}

pub async fn submit_contact_message(
    State(state): State<AppState>,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        INSERT INTO contact_messages
        (name, email, phone, company, subject, message)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(form.name.trim())
    .bind(form.email.trim().to_lowercase())
    .bind(form.phone)
    .bind(form.company)
    .bind(form.subject.trim())
    .bind(form.message.trim())
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Redirect::to("/contact?success=1").into_response(),
        Err(error) => {
            eprintln!("Failed to submit contact message: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit contact message",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_contact_message_show(Path(id): Path<Uuid>) -> impl IntoResponse {
    let _id = id;

    render(DashboardContactMessagesTemplate)
}

pub async fn dashboard_contact_message_reply(
    Path(id): Path<Uuid>,
    Form(form): Form<ReplyForm>,
) -> impl IntoResponse {
    let _id = id;
    let _reply = form.admin_reply;

    Redirect::to("/dashboard/contact-messages")
}
