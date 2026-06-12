use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewsletterSubscriberRecord {
    pub id: Uuid,
    pub email: String,
    pub source: Option<String>,
    pub subscribed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewsletterSubscriberEditorView {
    pub email: String,
    pub source: String,
}

impl NewsletterSubscriberRecord {
    pub fn admin_edit_url(&self) -> String {
        format!("/dashboard/newsletter-subscribers/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/newsletter-subscribers/{}/delete", self.id)
    }

    pub fn source_label(&self) -> &str {
        self.source
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Website")
    }

    pub fn subscribed_date_label(&self) -> String {
        self.subscribed_at.format("%d %b %Y").to_string()
    }

    pub fn to_editor_view(&self) -> NewsletterSubscriberEditorView {
        NewsletterSubscriberEditorView {
            email: self.email.clone(),
            source: self.source.clone().unwrap_or_default(),
        }
    }
}

impl NewsletterSubscriberEditorView {
    pub fn empty() -> Self {
        Self {
            email: String::new(),
            source: "Website".to_string(),
        }
    }
}
