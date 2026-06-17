use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    handlers::templates::{
        ServiceAreaCardView, ServiceAreaDetailFaqView, ServiceAreaDetailPointView,
        ServiceAreaPageView,
    },
    models::shared::slugify,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ServiceAreaRecord {
    pub id: Uuid,
    pub area_name: String,
    pub slug: String,
    pub area_type: String,
    pub country: String,
    pub market_region: String,
    pub short_description: String,
    pub overview: String,
    pub buyer_profile: Option<String>,
    pub delivery_focus: Option<String>,
    pub timezone_note: Option<String>,
    pub nearby_markets: Option<String>,
    pub hero_image_url: Option<String>,
    pub gallery_image_url_2: Option<String>,
    pub gallery_image_url_3: Option<String>,
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
pub struct ServiceAreaEditorView {
    pub area_name: String,
    pub slug: String,
    pub area_type: String,
    pub country: String,
    pub market_region: String,
    pub short_description: String,
    pub overview: String,
    pub buyer_profile: String,
    pub delivery_focus: String,
    pub timezone_note: String,
    pub nearby_markets: String,
    pub hero_image_url: String,
    pub gallery_image_url_2: String,
    pub gallery_image_url_3: String,
    pub sort_order: i32,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_url: String,
    pub og_image_url: String,
    pub featured: bool,
    pub published: bool,
}

impl ServiceAreaRecord {
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
        format!("/dashboard/service-areas/{}/edit", self.id)
    }

    pub fn admin_delete_url(&self) -> String {
        format!("/dashboard/service-areas/{}/delete", self.id)
    }

    pub fn summary(&self) -> String {
        truncate_text(&self.short_description, 160)
    }

    pub fn display_type(&self) -> &str {
        match self.area_type.as_str() {
            "country" => "Country",
            "state" => "State / Region",
            _ => "City",
        }
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

    pub fn nearby_markets_list(&self) -> Vec<String> {
        self.nearby_markets
            .as_deref()
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn image_gallery(&self) -> Vec<String> {
        [
            self.hero_image_url.as_deref(),
            self.gallery_image_url_2.as_deref(),
            self.gallery_image_url_3.as_deref(),
        ]
        .into_iter()
        .flatten()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
    }

    pub fn to_card_view(&self) -> ServiceAreaCardView {
        ServiceAreaCardView {
            region: self.market_region.clone(),
            title: format!("{}, {}", self.area_name, self.country),
            path: format!("/service-areas/{}", self.slug),
            summary: self.short_description.clone(),
            area_type_label: self.display_type().to_string(),
        }
    }

    pub fn to_editor_view(&self) -> ServiceAreaEditorView {
        ServiceAreaEditorView {
            area_name: self.area_name.clone(),
            slug: self.slug.clone(),
            area_type: self.area_type.clone(),
            country: self.country.clone(),
            market_region: self.market_region.clone(),
            short_description: self.short_description.clone(),
            overview: self.overview.clone(),
            buyer_profile: self.buyer_profile.clone().unwrap_or_default(),
            delivery_focus: self.delivery_focus.clone().unwrap_or_default(),
            timezone_note: self.timezone_note.clone().unwrap_or_default(),
            nearby_markets: self.nearby_markets.clone().unwrap_or_default(),
            hero_image_url: self.hero_image_url.clone().unwrap_or_default(),
            gallery_image_url_2: self.gallery_image_url_2.clone().unwrap_or_default(),
            gallery_image_url_3: self.gallery_image_url_3.clone().unwrap_or_default(),
            sort_order: self.sort_order,
            meta_title: self.meta_title.clone().unwrap_or_default(),
            meta_description: self.meta_description.clone().unwrap_or_default(),
            canonical_url: self.canonical_url.clone().unwrap_or_default(),
            og_image_url: self.og_image_url.clone().unwrap_or_default(),
            featured: self.featured,
            published: self.published,
        }
    }

    pub fn to_page_view(&self) -> ServiceAreaPageView {
        let city_or_area = &self.area_name;
        let country = &self.country;

        ServiceAreaPageView {
            city: city_or_area.clone(),
            region: self.market_region.clone(),
            title: format!("{}, {}", city_or_area, country),
            canonical_path: format!("/service-areas/{}", self.slug),
            meta_title: self
                .meta_title
                .clone()
                .unwrap_or_else(|| format!("Software Development in {} | LKProfessionals", city_or_area)),
            meta_description: self
                .meta_description
                .clone()
                .unwrap_or_else(|| self.short_description.clone()),
            hero_title: format!(
                "{} delivery support from LKProfessionals.",
                city_or_area
            ),
            hero_description: self.overview.clone(),
            positioning: self
                .delivery_focus
                .clone()
                .unwrap_or_else(|| format!("LKProfessionals supports businesses in {} with software delivery, web platforms, SEO, automation, and practical offshore execution from Sri Lanka.", city_or_area)),
            timezone_note: self
                .timezone_note
                .clone()
                .unwrap_or_else(|| "Delivery is structured around clear communication, reliable progress reporting, and practical overlap where needed.".to_string()),
            nearby_markets: self.nearby_markets_list(),
            buyer_points: vec![
                ServiceAreaDetailPointView {
                    title: format!("Why buyers in {} consider LKProfessionals", city_or_area),
                    description: self.short_description.clone(),
                },
                ServiceAreaDetailPointView {
                    title: "Commercial fit".to_string(),
                    description: self.buyer_profile.clone().unwrap_or_else(|| format!("This service area page is designed for companies in {} that need serious technical support without the cost structure of a large-city agency.", city_or_area)),
                },
                ServiceAreaDetailPointView {
                    title: "Delivery model".to_string(),
                    description: self
                        .delivery_focus
                        .clone()
                        .unwrap_or_else(|| "Projects are run with practical scoping, direct communication, and a focus on maintainable delivery.".to_string()),
                },
            ],
            service_points: vec![
                ServiceAreaDetailPointView {
                    title: "Custom software and internal systems".to_string(),
                    description: format!("Operational software, dashboards, portals, and workflow systems for teams in {}.", city_or_area),
                },
                ServiceAreaDetailPointView {
                    title: "Web platforms and lead-generation websites".to_string(),
                    description: format!("Conversion-focused websites, landing pages, and web applications aligned to commercial goals in {}.", city_or_area),
                },
                ServiceAreaDetailPointView {
                    title: "SEO, automation, and digital operations".to_string(),
                    description: "Technical SEO, answer-engine-ready content structure, automation workflows, and ongoing optimization support.".to_string(),
                },
            ],
            faqs: vec![
                ServiceAreaDetailFaqView {
                    question: format!("Does LKProfessionals work with businesses in {}?", city_or_area),
                    answer: format!("Yes. LKProfessionals supports businesses in {}, {} through remote delivery, structured communication, and long-term technical support.", city_or_area, country),
                },
                ServiceAreaDetailFaqView {
                    question: format!("What can LKProfessionals deliver for {} clients?", city_or_area),
                    answer: "The team can deliver custom software, websites, SEO systems, automation workflows, hosting guidance, and digital transformation support.".to_string(),
                },
                ServiceAreaDetailFaqView {
                    question: format!("Why use a Sri Lankan delivery partner for {} work?", city_or_area),
                    answer: "For many businesses, the benefit is a stronger cost-to-capability ratio, direct communication, and one team that can combine software, web, SEO, and operational thinking.".to_string(),
                },
                ServiceAreaDetailFaqView {
                    question: "Can these pages be customized for local market positioning?".to_string(),
                    answer: "Yes. Every service area entry is editable from the dashboard, including copy, SEO metadata, nearby markets, and image assets.".to_string(),
                },
            ],
            image_gallery: self.image_gallery(),
            area_type_label: self.display_type().to_string(),
        }
    }
}

impl ServiceAreaEditorView {
    pub fn empty() -> Self {
        Self {
            area_name: String::new(),
            slug: String::new(),
            area_type: "city".to_string(),
            country: String::new(),
            market_region: String::new(),
            short_description: String::new(),
            overview: String::new(),
            buyer_profile: String::new(),
            delivery_focus: String::new(),
            timezone_note: String::new(),
            nearby_markets: String::new(),
            hero_image_url: String::new(),
            gallery_image_url_2: String::new(),
            gallery_image_url_3: String::new(),
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
            self.area_name.as_str()
        } else {
            self.slug.as_str()
        };

        slugify(source)
    }
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
