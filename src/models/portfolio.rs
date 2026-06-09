use crate::models::shared::seo_fields::SeoFields; 
#[derive(Debug, Clone)] 
pub struct Portfolio {
    pub id: i32, 
    pub title: String, 
    pub client_name: Option<String>, 
    pub overview: String, 
    pub challenge: Option<String>, 
    pub solution: Option<String>, 
    pub results: Option<String>, 
    pub technologies: Option<String>, 
    pub live_url: Option<String>, 
    pub featured: bool, 
    pub seo: SeoFields, 
}