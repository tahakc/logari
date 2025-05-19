use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub media_type: MediaType,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Anime,
    Manga,
    Movie,
    TvShow,
    Game,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_results: u32,
    pub total_pages: u32,
    pub current_page: u32,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub rating: Option<f32>,
    pub description: Option<String>,
}
