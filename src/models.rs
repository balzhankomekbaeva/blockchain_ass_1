use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsArticle {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_at: chrono::NaiveDateTime,
    pub description: Option<String>,
}