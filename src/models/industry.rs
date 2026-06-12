use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::shared::slugify;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct IndustryRecord {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub overview: String,
    pub challenge_focus: Option<String>,
    pub solution_focus: Option<String>,
    pub icon_class: Option<String>,
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
pub struct IndustryCardView {
    pub title: String,
    pub short_description: String,
    pub overview: String,
    pub challenge_focus: Option<String>,
    pub solution_focus: Option<String>,
    pub icon_class: String,
}

#[derive(Debug, Clone)]
pub struct IndustryEditorView {
    pub title: String,
    pub slug: String,
    pub short_description: String,
    pub overview: String,
    pub challenge_focus: String,
    pub solution_focus: String,
    pub icon_class: String,
    pub sort_order: i32,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_url: String,
    pub og_image_url: String,
    pub featured: bool,
    pub published: bool,
}

impl IndustryRecord {
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

    pub fn feature_label(&self) -> &str {
        if self.featured {
            "Featured"
        } else {
            "Standard"
        }
    }

    pub fn feature_badge_class(&self) -> &str {
        if self.featured {
            "bg-cyan-50 text-cyan-700 ring-cyan-200"
        } else {
            "bg-slate-100 text-slate-600 ring-slate-200"
        }
    }

    pub fn admin_edit_url(&self) -> String {
        format!("/dashboard/industries/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/industries/{}/delete", self.id)
    }

    pub fn summary(&self) -> String {
        truncate_text(&self.short_description, 140)
    }

    pub fn icon_or_default(&self) -> String {
        self.icon_class
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "fa-solid fa-briefcase".to_string())
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

    pub fn to_card_view(&self) -> IndustryCardView {
        IndustryCardView {
            title: self.title.clone(),
            short_description: self.short_description.clone(),
            overview: self.overview.clone(),
            challenge_focus: clean_text(&self.challenge_focus),
            solution_focus: clean_text(&self.solution_focus),
            icon_class: self.icon_or_default(),
        }
    }

    pub fn to_editor_view(&self) -> IndustryEditorView {
        IndustryEditorView {
            title: self.title.clone(),
            slug: self.slug.clone(),
            short_description: self.short_description.clone(),
            overview: self.overview.clone(),
            challenge_focus: self.challenge_focus.clone().unwrap_or_default(),
            solution_focus: self.solution_focus.clone().unwrap_or_default(),
            icon_class: self.icon_class.clone().unwrap_or_default(),
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

impl IndustryEditorView {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            slug: String::new(),
            short_description: String::new(),
            overview: String::new(),
            challenge_focus: String::new(),
            solution_focus: String::new(),
            icon_class: String::new(),
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
        let source = if self.slug.trim().is_empty() {
            self.title.as_str()
        } else {
            self.slug.as_str()
        };

        slugify(source)
    }
}

fn clean_text(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
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
