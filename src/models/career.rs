use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::shared::slugify;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CareerRecord {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub team: Option<String>,
    pub location: String,
    pub workplace_mode: String,
    pub employment_type: String,
    pub experience_level: String,
    pub salary_range: Option<String>,
    pub summary: String,
    pub description: String,
    pub responsibilities: Option<String>,
    pub requirements: Option<String>,
    pub benefits: Option<String>,
    pub application_email: Option<String>,
    pub apply_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub featured: bool,
    pub published: bool,
    pub sort_order: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub og_image_url: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CareerCardView {
    pub title: String,
    pub team: String,
    pub location: String,
    pub workplace_mode: String,
    pub employment_type: String,
    pub experience_level: String,
    pub salary_range: String,
    pub summary: String,
    pub public_url: String,
}

#[derive(Debug, Clone)]
pub struct CareerDetailView {
    pub title: String,
    pub team: String,
    pub location: String,
    pub workplace_mode: String,
    pub employment_type: String,
    pub experience_level: String,
    pub salary_range: String,
    pub summary: String,
    pub description_paragraphs: Vec<String>,
    pub responsibilities: Vec<String>,
    pub requirements: Vec<String>,
    pub benefits: Vec<String>,
    pub application_email: String,
    pub apply_url: String,
    pub canonical_url: String,
    pub meta_title: String,
    pub meta_description: String,
    pub og_image_url: String,
    pub cover_image_url: String,
    pub published_date_iso: String,
    pub updated_date_iso: String,
}

#[derive(Debug, Clone)]
pub struct CareerEditorView {
    pub title: String,
    pub slug: String,
    pub team: String,
    pub location: String,
    pub workplace_mode: String,
    pub employment_type: String,
    pub experience_level: String,
    pub salary_range: String,
    pub summary: String,
    pub description: String,
    pub responsibilities: String,
    pub requirements: String,
    pub benefits: String,
    pub application_email: String,
    pub apply_url: String,
    pub cover_image_url: String,
    pub sort_order: i32,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_url: String,
    pub og_image_url: String,
    pub featured: bool,
    pub published: bool,
}

impl CareerRecord {
    pub fn public_url(&self) -> String {
        format!("/careers/{}", self.slug)
    }

    pub fn admin_edit_url(&self) -> String {
        format!("/dashboard/careers/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/careers/{}/delete", self.id)
    }

    pub fn team_label(&self) -> &str {
        self.team
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("General")
    }

    pub fn summary(&self) -> String {
        truncate_text(&self.summary, 170)
    }

    pub fn salary_label(&self) -> &str {
        self.salary_range
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Discussed during the hiring process")
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

    pub fn display_cover_image(&self) -> &str {
        self.cover_image_url
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or(
                "https://images.unsplash.com/photo-1522071820081-009f0129c71c?auto=format&fit=crop&w=1400&q=80",
            )
    }

    pub fn meta_title_or_fallback(&self) -> String {
        self.meta_title
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("{} Career Opportunity | LKProfessionals", self.title))
    }

    pub fn meta_description_or_fallback(&self) -> String {
        self.meta_description
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| truncate_text(&self.summary, 156))
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

    pub fn to_card_view(&self) -> CareerCardView {
        CareerCardView {
            title: self.title.clone(),
            team: self.team_label().to_string(),
            location: self.location.clone(),
            workplace_mode: self.workplace_mode.clone(),
            employment_type: self.employment_type.clone(),
            experience_level: self.experience_level.clone(),
            salary_range: self.salary_label().to_string(),
            summary: self.summary(),
            public_url: self.public_url(),
        }
    }

    pub fn to_detail_view(&self) -> CareerDetailView {
        let published = self.published_at.unwrap_or(self.created_at);

        CareerDetailView {
            title: self.title.clone(),
            team: self.team_label().to_string(),
            location: self.location.clone(),
            workplace_mode: self.workplace_mode.clone(),
            employment_type: self.employment_type.clone(),
            experience_level: self.experience_level.clone(),
            salary_range: self.salary_label().to_string(),
            summary: self.summary.clone(),
            description_paragraphs: split_paragraphs(&self.description),
            responsibilities: split_lines(self.responsibilities.as_deref().unwrap_or_default()),
            requirements: split_lines(self.requirements.as_deref().unwrap_or_default()),
            benefits: split_lines(self.benefits.as_deref().unwrap_or_default()),
            application_email: self
                .application_email
                .clone()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "careers@lkprofessionals.com".to_string()),
            apply_url: format!("{}/apply", self.public_url()),
            canonical_url: self.canonical_url_or_fallback(),
            meta_title: self.meta_title_or_fallback(),
            meta_description: self.meta_description_or_fallback(),
            og_image_url: self.og_image_or_fallback(),
            cover_image_url: self.display_cover_image().to_string(),
            published_date_iso: published.to_rfc3339(),
            updated_date_iso: self.updated_at.to_rfc3339(),
        }
    }

    pub fn to_editor_view(&self) -> CareerEditorView {
        CareerEditorView {
            title: self.title.clone(),
            slug: self.slug.clone(),
            team: self.team.clone().unwrap_or_default(),
            location: self.location.clone(),
            workplace_mode: self.workplace_mode.clone(),
            employment_type: self.employment_type.clone(),
            experience_level: self.experience_level.clone(),
            salary_range: self.salary_range.clone().unwrap_or_default(),
            summary: self.summary.clone(),
            description: self.description.clone(),
            responsibilities: self.responsibilities.clone().unwrap_or_default(),
            requirements: self.requirements.clone().unwrap_or_default(),
            benefits: self.benefits.clone().unwrap_or_default(),
            application_email: self.application_email.clone().unwrap_or_default(),
            apply_url: self.apply_url.clone().unwrap_or_default(),
            cover_image_url: self.cover_image_url.clone().unwrap_or_default(),
            sort_order: self.sort_order,
            meta_title: self.meta_title.clone().unwrap_or_default(),
            meta_description: self.meta_description.clone().unwrap_or_default(),
            canonical_url: self.canonical_url.clone().unwrap_or_default(),
            og_image_url: self.og_image_url.clone().unwrap_or_default(),
            featured: self.featured,
            published: self.published,
        }
    }
}

impl CareerEditorView {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            slug: String::new(),
            team: "Engineering".to_string(),
            location: "Jaffna, Sri Lanka".to_string(),
            workplace_mode: "Hybrid".to_string(),
            employment_type: "Full-time".to_string(),
            experience_level: "Mid-level".to_string(),
            salary_range: String::new(),
            summary: String::new(),
            description: String::new(),
            responsibilities: String::new(),
            requirements: String::new(),
            benefits: String::new(),
            application_email: "careers@lkprofessionals.com".to_string(),
            apply_url: String::new(),
            cover_image_url: String::new(),
            sort_order: 0,
            meta_title: String::new(),
            meta_description: String::new(),
            canonical_url: String::new(),
            og_image_url: String::new(),
            featured: false,
            published: false,
        }
    }

    pub fn normalized_slug(&self) -> String {
        if self.slug.trim().is_empty() {
            slugify(&self.title)
        } else {
            slugify(&self.slug)
        }
    }
}

fn split_paragraphs(value: &str) -> Vec<String> {
    value
        .split("\n\n")
        .map(str::trim)
        .filter(|paragraph| !paragraph.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn split_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }

    let truncated = trimmed.chars().take(max_chars).collect::<String>();
    format!("{}...", truncated.trim_end())
}
