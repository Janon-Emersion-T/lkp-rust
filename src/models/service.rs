use crate::models::shared::seo_fields::SeoFields; 

#[derive(Debug, Clone)]
pub struct Service {
    pub id: i32,
    pub title: String,
    pub short_description: String,
    pub content: String,
    pub icon: Option<String>,
    pub featured: bool,
    pub seo: SeoFields, 
}