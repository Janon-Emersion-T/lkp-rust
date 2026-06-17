use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::shared::slugify;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct InsightRecord {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub cover_image_url: Option<String>,
    pub featured: bool,
    pub published: bool,
    pub view_count: i32,
    pub reading_time_minutes: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub og_image_url: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct InsightCardView {
    pub title: String,
    pub excerpt: String,
    pub author: String,
    pub category: String,
    pub cover_image_url: String,
    pub public_url: String,
    pub reading_time_label: String,
    pub published_date_label: String,
}

#[derive(Debug, Clone)]
pub struct InsightDetailView {
    pub title: String,
    pub excerpt: String,
    pub content_paragraphs: Vec<String>,
    pub author: String,
    pub category: String,
    pub cover_image_url: String,
    pub canonical_url: String,
    pub meta_title: String,
    pub meta_description: String,
    pub og_image_url: String,
    pub reading_time_label: String,
    pub published_date_iso: String,
    pub published_date_label: String,
    pub updated_date_iso: String,
}

#[derive(Debug, Clone)]
pub struct InsightEditorView {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub author: String,
    pub category: String,
    pub cover_image_url: String,
    pub reading_time_minutes: i32,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_url: String,
    pub og_image_url: String,
    pub featured: bool,
    pub published: bool,
}

impl InsightRecord {
    pub fn public_url(&self) -> String {
        format!("/insights/{}", self.slug)
    }

    pub fn admin_edit_url(&self) -> String {
        format!("/dashboard/insights/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/insights/{}/delete", self.id)
    }

    pub fn category_label(&self) -> &str {
        self.category
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Strategy")
    }

    pub fn status_label(&self) -> &str {
        if self.published { "Published" } else { "Draft" }
    }

    pub fn status_badge_class(&self) -> &str {
        if self.published {
            "bg-emerald-50 text-emerald-700 ring-emerald-200"
        } else {
            "bg-amber-50 text-amber-700 ring-amber-200"
        }
    }

    pub fn feature_badge_class(&self) -> &str {
        if self.featured {
            "bg-cyan-50 text-cyan-700 ring-cyan-200"
        } else {
            "bg-slate-100 text-slate-600 ring-slate-200"
        }
    }

    pub fn feature_label(&self) -> &str {
        if self.featured {
            "Featured"
        } else {
            "Standard"
        }
    }

    pub fn summary(&self) -> String {
        truncate_text(&self.excerpt, 150)
    }

    pub fn display_cover_image(&self) -> &str {
        if let Some(value) = self
            .cover_image_url
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            return value;
        }

        match self
            .category
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "seo" => {
                "https://images.unsplash.com/photo-1432888622747-4eb9a8efeb07?auto=format&fit=crop&w=1200&q=80"
            }
            "automation" => {
                "https://images.unsplash.com/photo-1518770660439-4636190af475?auto=format&fit=crop&w=1200&q=80"
            }
            "artificial intelligence" | "ai" => {
                "https://images.unsplash.com/photo-1677442136019-21780ecad995?auto=format&fit=crop&w=1200&q=80"
            }
            "digital marketing" => {
                "https://images.unsplash.com/photo-1460925895917-afdab827c52f?auto=format&fit=crop&w=1200&q=80"
            }
            "software development" => {
                "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=1200&q=80"
            }
            _ => {
                "https://images.unsplash.com/photo-1516321318423-f06f85e504b3?auto=format&fit=crop&w=1200&q=80"
            }
        }
    }

    pub fn meta_title_or_fallback(&self) -> String {
        self.meta_title
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("{} | LKProfessionals Insights", self.title))
    }

    pub fn meta_description_or_fallback(&self) -> String {
        self.meta_description
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| truncate_text(&self.excerpt, 158))
    }

    pub fn canonical_url_or_fallback(&self) -> String {
        self.canonical_url
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("https://lkprofessionals.com{}", self.public_url()))
    }

    pub fn og_image_or_fallback(&self) -> String {
        self.og_image_url
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.display_cover_image().to_string())
    }

    pub fn reading_time_label(&self) -> String {
        format!("{} min read", self.reading_time_minutes.max(1))
    }

    pub fn published_label(&self) -> String {
        self.published_at
            .unwrap_or(self.created_at)
            .format("%d %b %Y")
            .to_string()
    }

    pub fn updated_label(&self) -> String {
        self.updated_at.format("%d %b %Y").to_string()
    }

    pub fn view_count_label(&self) -> String {
        format_number(self.view_count)
    }

    pub fn to_card_view(&self) -> InsightCardView {
        let published = self.published_at.unwrap_or(self.created_at);

        InsightCardView {
            title: self.title.clone(),
            excerpt: self.summary(),
            author: self.author.clone(),
            category: self.category_label().to_string(),
            cover_image_url: self.display_cover_image().to_string(),
            public_url: self.public_url(),
            reading_time_label: self.reading_time_label(),
            published_date_label: published.format("%d %b %Y").to_string(),
        }
    }

    pub fn to_detail_view(&self) -> InsightDetailView {
        let published = self.published_at.unwrap_or(self.created_at);

        InsightDetailView {
            title: self.title.clone(),
            excerpt: self.excerpt.clone(),
            content_paragraphs: split_paragraphs(&self.content),
            author: self.author.clone(),
            category: self.category_label().to_string(),
            cover_image_url: self.display_cover_image().to_string(),
            canonical_url: self.canonical_url_or_fallback(),
            meta_title: self.meta_title_or_fallback(),
            meta_description: self.meta_description_or_fallback(),
            og_image_url: self.og_image_or_fallback(),
            reading_time_label: self.reading_time_label(),
            published_date_iso: published.to_rfc3339(),
            published_date_label: published.format("%d %B %Y").to_string(),
            updated_date_iso: self.updated_at.to_rfc3339(),
        }
    }

    pub fn to_editor_view(&self) -> InsightEditorView {
        InsightEditorView {
            title: self.title.clone(),
            slug: self.slug.clone(),
            excerpt: self.excerpt.clone(),
            content: self.content.clone(),
            author: self.author.clone(),
            category: self.category.clone().unwrap_or_default(),
            cover_image_url: self.cover_image_url.clone().unwrap_or_default(),
            reading_time_minutes: self.reading_time_minutes,
            meta_title: self.meta_title.clone().unwrap_or_default(),
            meta_description: self.meta_description.clone().unwrap_or_default(),
            canonical_url: self.canonical_url.clone().unwrap_or_default(),
            og_image_url: self.og_image_url.clone().unwrap_or_default(),
            featured: self.featured,
            published: self.published,
        }
    }
}

impl InsightEditorView {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            slug: String::new(),
            excerpt: String::new(),
            content: String::new(),
            author: "LKProfessionals Team".to_string(),
            category: String::new(),
            cover_image_url: String::new(),
            reading_time_minutes: 5,
            meta_title: String::new(),
            meta_description: String::new(),
            canonical_url: String::new(),
            og_image_url: String::new(),
            featured: false,
            published: false,
        }
    }

    pub fn normalized_slug(&self) -> String {
        let source = if self.slug.trim().is_empty() {
            self.title.as_str()
        } else {
            self.slug.as_str()
        };

        slugify(source)
    }
}

fn split_paragraphs(content: &str) -> Vec<String> {
    content
        .split("\n\n")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let mut output = String::new();

    for (index, ch) in value.trim().chars().enumerate() {
        if index >= max_chars {
            output.push_str("...");
            return output;
        }

        output.push(ch);
    }

    output
}

fn format_number(value: i32) -> String {
    let digits = value.max(0).to_string();
    let mut formatted = String::with_capacity(digits.len() + (digits.len() / 3));

    for (index, character) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(character);
    }

    formatted.chars().rev().collect()
}
