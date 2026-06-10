use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ContactMessage {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub service_interest: Option<String>,
    pub budget_range: Option<String>,
    pub project_timeline: Option<String>,
    pub subject: String,
    pub message: String,
    pub source: String,
    pub status: String,
    pub priority: String,
    pub lead_score: i32,
    pub admin_reply: Option<String>,
    pub replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LeadStats {
    pub total: i64,
    pub new_count: i64,
    pub contacted_count: i64,
    pub qualified_count: i64,
    pub converted_count: i64,
}