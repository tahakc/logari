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
    pub slug: Option<String>,
    pub rating_top: Option<i32>,
    pub metacritic: Option<u32>,
    pub playtime: Option<i32>,
    pub genres: Option<Vec<Genre>>,
    pub platforms: Option<Vec<Platform>>,
    pub esrb_rating: Option<EsrbRating>,
    pub tags: Option<Vec<Tag>>,
    pub screenshots: Option<Vec<Screenshot>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialie)]
pub struct EsrbRating {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screenshot {
    pub id: i32,
    pub name: String,
}
