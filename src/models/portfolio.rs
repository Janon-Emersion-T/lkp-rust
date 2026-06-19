use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::shared::slugify;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PortfolioRecord {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub client_name: Option<String>,
    pub industry: Option<String>,
    pub service_category: Option<String>,
    pub excerpt: String,
    pub overview: String,
    pub challenge: Option<String>,
    pub solution: Option<String>,
    pub results: Option<String>,
    pub impact_metrics: Option<String>,
    pub technologies: Option<String>,
    pub testimonial_quote: Option<String>,
    pub testimonial_author: Option<String>,
    pub testimonial_author_role: Option<String>,
    pub cover_image_url: Option<String>,
    pub live_url: Option<String>,
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
pub struct PortfolioCardView {
    pub title: String,
    pub client_name: String,
    pub industry: String,
    pub service_category: String,
    pub excerpt: String,
    pub cover_image_url: String,
    pub public_url: String,
    pub technology_tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PortfolioDetailView {
    pub title: String,
    pub client_name: String,
    pub industry: String,
    pub service_category: String,
    pub excerpt: String,
    pub overview: String,
    pub challenge: Option<String>,
    pub solution: Option<String>,
    pub results: Vec<String>,
    pub impact_metrics: Vec<String>,
    pub technology_tags: Vec<String>,
    pub testimonial_quote: Option<String>,
    pub testimonial_author: Option<String>,
    pub testimonial_author_role: Option<String>,
    pub cover_image_url: String,
    pub live_url: Option<String>,
    pub canonical_url: String,
    pub meta_title: String,
    pub meta_description: String,
    pub og_image_url: String,
    pub published_date_iso: String,
    pub published_date_label: String,
    pub updated_date_iso: String,
}

#[derive(Debug, Clone)]
pub struct PortfolioEditorView {
    pub title: String,
    pub slug: String,
    pub client_name: String,
    pub industry: String,
    pub service_category: String,
    pub excerpt: String,
    pub overview: String,
    pub challenge: String,
    pub solution: String,
    pub results: String,
    pub impact_metrics: String,
    pub technologies: String,
    pub testimonial_quote: String,
    pub testimonial_author: String,
    pub testimonial_author_role: String,
    pub cover_image_url: String,
    pub live_url: String,
    pub sort_order: i32,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_url: String,
    pub og_image_url: String,
    pub featured: bool,
    pub published: bool,
}

impl PortfolioRecord {
    pub fn public_url(&self) -> String {
        format!("/case-studies/{}", self.slug)
    }

    pub fn admin_edit_url(&self) -> String {
        format!("/dashboard/portfolios/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/portfolios/{}/delete", self.id)
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

    pub fn industry_label(&self) -> &str {
        self.industry
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Multi-industry")
    }

    pub fn service_category_label(&self) -> &str {
        self.service_category
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Digital Delivery")
    }

    pub fn client_label(&self) -> &str {
        self.client_name
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Confidential Client")
    }

    pub fn summary(&self) -> String {
        truncate_text(&self.excerpt, 140)
    }

    pub fn technologies_preview(&self) -> Vec<String> {
        split_tokens(self.technologies.as_deref())
            .into_iter()
            .take(4)
            .collect()
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
            .industry
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "healthcare" => {
                "https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?auto=format&fit=crop&w=1200&q=80"
            }
            "education" => {
                "https://images.unsplash.com/photo-1522202176988-66273c2fd55f?auto=format&fit=crop&w=1200&q=80"
            }
            "retail" | "retail & commerce" | "commerce" => {
                "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?auto=format&fit=crop&w=1200&q=80"
            }
            "logistics" | "logistics & distribution" => {
                "https://images.unsplash.com/photo-1586528116311-ad8dd3c8310d?auto=format&fit=crop&w=1200&q=80"
            }
            _ => match self
                .service_category
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase()
                .as_str()
            {
                "web development" => {
                    "https://images.unsplash.com/photo-1460925895917-afdab827c52f?auto=format&fit=crop&w=1200&q=80"
                }
                "software development" | "custom software development" => {
                    "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=1200&q=80"
                }
                "ai & automation" | "automation" => {
                    "https://images.unsplash.com/photo-1485827404703-89b55fcc595e?auto=format&fit=crop&w=1200&q=80"
                }
                _ => {
                    "https://images.unsplash.com/photo-1516321318423-f06f85e504b3?auto=format&fit=crop&w=1200&q=80"
                }
            },
        }
    }

    pub fn meta_title_or_fallback(&self) -> String {
        self.meta_title
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("{} Case Study | LKProfessionals", self.title))
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

    pub fn to_card_view(&self) -> PortfolioCardView {
        PortfolioCardView {
            title: self.title.clone(),
            client_name: self.client_label().to_string(),
            industry: self.industry_label().to_string(),
            service_category: self.service_category_label().to_string(),
            excerpt: self.summary(),
            cover_image_url: self.display_cover_image().to_string(),
            public_url: self.public_url(),
            technology_tags: self.technologies_preview(),
        }
    }

    pub fn to_detail_view(&self) -> PortfolioDetailView {
        let published_date = self.published_at.unwrap_or(self.created_at);

        PortfolioDetailView {
            title: self.title.clone(),
            client_name: self.client_label().to_string(),
            industry: self.industry_label().to_string(),
            service_category: self.service_category_label().to_string(),
            excerpt: self.excerpt.clone(),
            overview: self.overview.clone(),
            challenge: clean_text(&self.challenge),
            solution: clean_text(&self.solution),
            results: split_lines(self.results.as_deref()),
            impact_metrics: split_lines(self.impact_metrics.as_deref()),
            technology_tags: split_tokens(self.technologies.as_deref()),
            testimonial_quote: clean_text(&self.testimonial_quote),
            testimonial_author: clean_text(&self.testimonial_author),
            testimonial_author_role: clean_text(&self.testimonial_author_role),
            cover_image_url: self.display_cover_image().to_string(),
            live_url: clean_text(&self.live_url),
            canonical_url: self.canonical_url_or_fallback(),
            meta_title: self.meta_title_or_fallback(),
            meta_description: self.meta_description_or_fallback(),
            og_image_url: self.og_image_or_fallback(),
            published_date_iso: published_date.to_rfc3339(),
            published_date_label: published_date.format("%d %B %Y").to_string(),
            updated_date_iso: self.updated_at.to_rfc3339(),
        }
    }

    pub fn to_editor_view(&self) -> PortfolioEditorView {
        PortfolioEditorView {
            title: self.title.clone(),
            slug: self.slug.clone(),
            client_name: self.client_name.clone().unwrap_or_default(),
            industry: self.industry.clone().unwrap_or_default(),
            service_category: self.service_category.clone().unwrap_or_default(),
            excerpt: self.excerpt.clone(),
            overview: self.overview.clone(),
            challenge: self.challenge.clone().unwrap_or_default(),
            solution: self.solution.clone().unwrap_or_default(),
            results: self.results.clone().unwrap_or_default(),
            impact_metrics: self.impact_metrics.clone().unwrap_or_default(),
            technologies: self.technologies.clone().unwrap_or_default(),
            testimonial_quote: self.testimonial_quote.clone().unwrap_or_default(),
            testimonial_author: self.testimonial_author.clone().unwrap_or_default(),
            testimonial_author_role: self.testimonial_author_role.clone().unwrap_or_default(),
            cover_image_url: self.cover_image_url.clone().unwrap_or_default(),
            live_url: self.live_url.clone().unwrap_or_default(),
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

impl PortfolioEditorView {
    pub fn empty() -> Self {
        Self {
            title: String::new(),
            slug: String::new(),
            client_name: String::new(),
            industry: String::new(),
            service_category: String::new(),
            excerpt: String::new(),
            overview: String::new(),
            challenge: String::new(),
            solution: String::new(),
            results: String::new(),
            impact_metrics: String::new(),
            technologies: String::new(),
            testimonial_quote: String::new(),
            testimonial_author: String::new(),
            testimonial_author_role: String::new(),
            cover_image_url: String::new(),
            live_url: String::new(),
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

fn split_lines(value: Option<&str>) -> Vec<String> {
    value
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn split_tokens(value: Option<&str>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(|ch| ch == '\n' || ch == ',')
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
