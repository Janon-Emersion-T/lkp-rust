use crate::models::shared::seo_fields::SeoFields;

#[derive(Debug, Clone)]
pub struct Insight {
    pub id: i32,

    pub title: String,
    pub excerpt: String,
    pub content: String,

    pub author: String,

    pub category_id: i32,

    pub featured: bool,

    pub reading_time: i32,

    pub views: i64,

    pub seo: SeoFields,
}