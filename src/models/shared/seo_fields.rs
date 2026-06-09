use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SeoFields {
    pub slug: String,
    
    pub meta_title: String,
    pub meta_description: String,
    pub meta_keywords: Option<String>,
    
    pub canonical_url: Option<String>,
    
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
    
    pub schema_type: Option<String>,
    pub structured_data_json: Option<String>,
    
    pub robots_index: bool,
    pub robots_follow: bool,
    
    pub featured_image: Option<String>,
    
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>, 
}