use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewsletterCampaignRecord {
    pub id: Uuid,
    pub title: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub status: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewsletterCampaignEditorView {
    pub title: String,
    pub subject: String,
    pub preview_text: String,
    pub content_html: String,
    pub cta_label: String,
    pub cta_url: String,
}

impl NewsletterCampaignRecord {
    pub fn status_badge_class(&self) -> &str {
        match self.status.as_str() {
            "sent" => "bg-emerald-50 text-emerald-700 ring-emerald-200",
            "sending" => "bg-cyan-50 text-cyan-700 ring-cyan-200",
            "failed" => "bg-rose-50 text-rose-700 ring-rose-200",
            _ => "bg-amber-50 text-amber-700 ring-amber-200",
        }
    }

    pub fn status_label(&self) -> &str {
        match self.status.as_str() {
            "sent" => "Sent",
            "sending" => "Sending",
            "failed" => "Attention",
            _ => "Queued",
        }
    }

    pub fn source_label(&self) -> &str {
        match self.source_type.as_str() {
            "insight" => "Insight Auto Send",
            "portfolio" => "Portfolio Auto Send",
            _ => "Manual Newsletter",
        }
    }

    pub fn sent_date_label(&self) -> String {
        self.sent_at
            .unwrap_or(self.created_at)
            .format("%d %b %Y")
            .to_string()
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/newsletters/{}/delete", self.id)
    }

    pub fn is_manual(&self) -> bool {
        self.source_type == "manual"
    }
}

impl NewsletterCampaignEditorView {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            subject: String::new(),
            preview_text: String::new(),
            content_html: String::new(),
            cta_label: "Read More".to_string(),
            cta_url: String::new(),
        }
    }
}
