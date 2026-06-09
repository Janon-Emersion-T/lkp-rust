use crate::models::shared::seo_fields::SeoFields; 
#[derive(Debug, Clone)] 
pub struct Career { 
    pub id: i32, 
    pub title: String, 
    pub employment_type: String, 
    pub location: String, 
    pub experience_level: String, 
    pub salary_range: Option<String>, 
    pub description: String, 
    pub responsibilities: String, 
    pub requirements: String, 
    pub benefits: Option<String>, 
    pub featured: bool, 
    pub seo: SeoFields, 
}