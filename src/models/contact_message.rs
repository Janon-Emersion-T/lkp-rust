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

impl ContactMessage {
    pub fn whatsapp_url(&self) -> Option<String> {
        let phone = self.phone.as_deref()?.trim();
        let digits: String = phone.chars().filter(|ch| ch.is_ascii_digit()).collect();

        if digits.is_empty() {
            return None;
        }

        Some(format!("https://wa.me/{digits}"))
    }

    pub fn service_label(&self) -> &str {
        match self.service_interest.as_deref() {
            Some("web_development") => "Website Development",
            Some("custom_software") => "Custom Software",
            Some("pos_erp") => "POS / ERP System",
            Some("seo") => "SEO & Search Growth",
            Some("digital_marketing") => "Digital Marketing",
            Some("hosting_domain") => "Hosting & Domain",
            Some("it_consultation") => "IT Consultation",
            Some("ai_automation") => "AI & Automation",
            Some(_) => "Other Service",
            None => "General Inquiry",
        }
    }

    pub fn budget_label(&self) -> &str {
        match self.budget_range.as_deref() {
            Some("below_100") => "Below USD 100",
            Some("100_500") => "USD 100 - 500",
            Some("500_1000") => "USD 500 - 1,000",
            Some("1000_plus") => "USD 1,000+",
            Some("enterprise") => "Enterprise / Long-term",
            Some(_) => "Custom Budget",
            None => "Not Given",
        }
    }

    pub fn timeline_label(&self) -> &str {
        match self.project_timeline.as_deref() {
            Some("urgent") => "Urgent",
            Some("this_month") => "This Month",
            Some("1_3_months") => "1 - 3 Months",
            Some("planning") => "Still Planning",
            Some(_) => "Custom Timeline",
            None => "Not Given",
        }
    }

    pub fn status_label(&self) -> &str {
        match self.status.as_str() {
            "new" => "New",
            "contacted" => "Contacted",
            "qualified" => "Qualified",
            "converted" => "Converted",
            "archived" => "Archived",
            "spam" => "Spam",
            _ => "Unknown",
        }
    }

    pub fn priority_label(&self) -> &str {
        match self.priority.as_str() {
            "high" => "High",
            "medium" => "Medium",
            "normal" => "Normal",
            _ => "Normal",
        }
    }

    pub fn priority_badge_class(&self) -> &str {
        match self.priority.as_str() {
            "high" => "bg-red-50 text-red-700 ring-red-200",
            "medium" => "bg-amber-50 text-amber-700 ring-amber-200",
            _ => "bg-slate-100 text-slate-700 ring-slate-200",
        }
    }

    pub fn status_badge_class(&self) -> &str {
        match self.status.as_str() {
            "new" => "bg-cyan-50 text-cyan-700 ring-cyan-200",
            "contacted" => "bg-blue-50 text-blue-700 ring-blue-200",
            "qualified" => "bg-amber-50 text-amber-700 ring-amber-200",
            "converted" => "bg-emerald-50 text-emerald-700 ring-emerald-200",
            "archived" => "bg-slate-100 text-slate-600 ring-slate-200",
            "spam" => "bg-red-50 text-red-700 ring-red-200",
            _ => "bg-slate-100 text-slate-700 ring-slate-200",
        }
    }

    pub fn score_bar_width(&self) -> i32 {
        self.lead_score.clamp(0, 100)
    }

    pub fn company_label(&self) -> &str {
        match self.company.as_deref() {
            Some(company) if !company.trim().is_empty() => company,
            _ => "No company provided",
        }
    }

    pub fn next_follow_up_input_value(&self) -> Option<String> {
        self.next_follow_up_at
            .as_ref()
            .map(|value| value.format("%Y-%m-%dT%H:%M").to_string())
    }

    pub fn source_label(&self) -> &str {
        match self.source.as_str() {
            "contact_page" => "Contact Page",
            other if !other.trim().is_empty() => other,
            _ => "Unknown",
        }
    }

    pub fn note_preview(&self) -> Option<&str> {
        self.admin_reply
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .or(self
                .internal_note
                .as_deref()
                .filter(|value| !value.trim().is_empty()))
    }
}
