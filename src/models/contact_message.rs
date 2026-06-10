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
    pub internal_note: Option<String>,
    pub assigned_to: Option<String>,
    pub lost_reason: Option<String>,
    pub next_follow_up_at: Option<DateTime<Utc>>,
    pub contacted_at: Option<DateTime<Utc>>,
    pub qualified_at: Option<DateTime<Utc>>,
    pub converted_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub spam_at: Option<DateTime<Utc>>,
    pub replied_at: Option<DateTime<Utc>>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
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
    pub archived_count: i64,
    pub spam_count: i64,
    pub high_priority_count: i64,
    pub avg_score: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct LeadFilters {
    pub status: String,
    pub priority: String,
    pub service: String,
    pub q: String,
}

impl LeadFilters {
    pub fn is_active(&self) -> bool {
        self.status != "all"
            || self.priority != "all"
            || self.service != "all"
            || !self.q.trim().is_empty()
    }
}
