use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use serde_json::Value;

use crate::error::{AppError, Result};
use crate::models::search::{MediaType, SearchResponse, SearchResult};
use super::ApiClient;

const RAWG_API_URL: &str = "https://api.rawg.io/api";

#[derive(Debug, Deserialize)]
struct RawgSearchResponse {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<RawgGame>,
}

#[derive(Debug, Deserialize)]
struct RawgGame {
    id: u32,
    name: String,
    background_image: Option<String>,
    released: Option<String>,
    rating: Option<f32>,
    #[serde(default)]
    slug: Option<String>,
    #[serde(default)]
    metacritic: Option<u32>,
}

impl ApiClient {
    pub async fn search_rawg(&self, query: &str, page: u32) -> Result<SearchResponse> {
        let url = format!("{}/games", RAWG_API_URL);

        let response = self.client
            .get(&url)
            .query(&[
                ("key", &self.config.api.rawg_api_key),
                ("search", &query.to_string()),
                ("page", &page.to_string()),
                ("page_size", &"20".to_string()),
            ])
            .send()
            .await?
            .json::<RawgSearchResponse>()
            .await?;

        let results = response.results
            .into_iter()
            .map(|game| {
                let description = Some(format!(
                    "Game ID: {}, {}{}",
                    game.id,
                    if let Some(mc) = game.metacritic {
                        format!("Metacritic: {}/100, ", mc)
                    } else {
                        "".to_string()
                    },
                    if let Some(slug) = game.slug {
                        format!("Slug: {}", slug)
                    } else {
                        "".to_string()
                    }
                ));

                SearchResult {
                    id: game.id.to_string(),
                    title: game.name,
                    media_type: MediaType::Game,
                    poster_path: game.background_image,
                    release_date: game.released,
                    rating: game.rating,
                    description,
                }
            })
            .collect();

        let total_results = response.count;
        let total_pages = (total_results + 19) / 20; // with page size 20
        
        Ok(SearchResponse {
            results,
            total_results,
            total_pages,
            current_page: page,
        })
    }
}
