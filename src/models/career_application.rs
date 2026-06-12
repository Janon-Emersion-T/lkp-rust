use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CareerApplicationRecord {
    pub id: Uuid,
    pub career_id: Option<Uuid>,
    pub role_title_snapshot: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub resume_url: Option<String>,
    pub cover_letter: String,
    pub experience_summary: Option<String>,
    pub availability: Option<String>,
    pub expected_salary: Option<String>,
    pub source: Option<String>,
    pub status: String,
    pub internal_notes: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CareerApplicationEditorView {
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin_url: String,
    pub portfolio_url: String,
    pub resume_url: String,
    pub cover_letter: String,
    pub experience_summary: String,
    pub availability: String,
    pub expected_salary: String,
}

impl CareerApplicationRecord {
    pub fn admin_show_url(&self) -> String {
        format!("/dashboard/career-applications/{}", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/career-applications/{}/delete", self.id)
    }

    pub fn status_badge_class(&self) -> &str {
        match self.status.as_str() {
            "reviewing" => "bg-cyan-50 text-cyan-700 ring-cyan-200",
            "shortlisted" => "bg-emerald-50 text-emerald-700 ring-emerald-200",
            "interview" => "bg-violet-50 text-violet-700 ring-violet-200",
            "closed" => "bg-slate-100 text-slate-600 ring-slate-200",
            "rejected" => "bg-rose-50 text-rose-700 ring-rose-200",
            _ => "bg-amber-50 text-amber-700 ring-amber-200",
        }
    }

    pub fn status_label(&self) -> &str {
        match self.status.as_str() {
            "reviewing" => "Reviewing",
            "shortlisted" => "Shortlisted",
            "interview" => "Interview",
            "closed" => "Closed",
            "rejected" => "Rejected",
            _ => "New",
        }
    }

    pub fn submitted_date_label(&self) -> String {
        self.created_at.format("%d %b %Y").to_string()
    }
}

impl CareerApplicationEditorView {
    pub fn empty() -> Self {
        Self {
            full_name: String::new(),
            email: String::new(),
            phone: String::new(),
            location: String::new(),
            linkedin_url: String::new(),
            portfolio_url: String::new(),
            resume_url: String::new(),
            cover_letter: String::new(),
            experience_summary: String::new(),
            availability: String::new(),
            expected_salary: String::new(),
        }
    }
}
