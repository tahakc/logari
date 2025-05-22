use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use serde_json::Value;

use crate::error::{AppError, Result};
use crate::models::search::{MediaType, SearchResponse, SearchResult, Genre, Platform, EsrbRating, Tag, Screenshot};
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
    rating_top: Option<i32>,
    playtime: Option<i32>,
    #[serde(default)]
    genres: Vec<RawgGenre>,
    #[serde(default)]
    platforms: Vec<RawgPlatformWrapper>,
    esrb_rating: Option<RawgEsrbRating>,
    #[serde(default)]
    tags: Vec<RawgTag>,
    #[serde(default)]
    short_screenshots: Vec<RawgScreenshot>,
}

#[derive(Debug, Deserialize)]
struct RawgGenre {
    id: i32,
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct RawgPlatformWrapper {
    platform: RawgPlatform,
}

#[derive(Debug, Deserialize)]
struct RawgPlatform {
    id: i32,
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct RawgEsrbRating {
    id: i32,
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct RawgTag {
    id: i32,
    name: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct RawgScreenshot {
    id: i32,
    image: String,
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
                SearchResult {
                    id: game.id.to_string(),
                    title: game.name,
                    media_type: MediaType::Game,
                    poster_path: game.background_image,
                    release_date: game.released,
                    rating: game.rating,
                    description: None,
                    slug: game.slug,
                    rating_top: game.rating_top,
                    metacritic: game.metacritic,
                    playtime: game.playtime,
                    genres: Some(game.genres.into_iter().map(|g| Genre {
                        id: g.id,
                        name: g.name,
                        slug: g.slug,
                    }).collect()),
                    platforms: Some(game.platforms.into_iter().map(|p| Platform {
                        id: p.platform.id,
                        name: p.platform.name,
                        slug: p.platform.slug,
                    }).collect()),
                    esrb_rating: game.esrb_rating.map(|e| EsrbRating {
                        id: e.id,
                        name: e.name,
                        slug: e.slug,
                    }),
                    tags: Some(game.tags.into_iter().map(|t| Tag {
                        id: t.id,
                        name: t.name,
                        slug: t.slug,
                    }).collect()),
                    screenshots: Some(game.short_screenshots.into_iter().map(|s| Screenshot {
                        id: s.id,
                        image: s.image,
                    }).collect()),
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

    pub async fn get_game_details(&self, game_id: String) -> Result<Value> {
        let url = format!("{}/games/{}", RAWG_API_URL, game_id);

        let response = self.client
            .get(&url)
            .query(&[
                ("key", &self.config.api.rawg_api_key),
            ])
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    pub async fn get_popular_games(&self, page: u32) -> Result<Value> {
        let url = format!("{}/games", RAWG_API_URL);

        let response = self.client
            .get(&url)
            .query(&[
                ("key", &self.config.api.rawg_api_key),
                ("ordering", &"-rating".to_string()),
                ("page", &page.to_string()),
                ("page_size", &"20".to_string()),
            ])
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    pub async fn get_new_releases(&self, page: u32) -> Result<Value> {
        let now = Utc::now();
        let thirty_days_ago = now - Duration::days(30);

        let url = format!("{}/games", RAWG_API_URL);

        let response = self.client
            .get(&url)
            .query(&[
                ("key", &self.config.api.rawg_api_key),
                ("dates", &format!("{},{}", thirty_days_ago.format("%Y-%m-%d"), now.format("%Y-%m-%d"))),
                ("ordering", &"-added".to_string()),
                ("page", &page.to_string()),
                ("page_size", &"20".to_string()),
            ])
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    pub async fn get_upcoming_games(&self, page: u32) -> Result<Value> {
        let now = Utc::now();
        let six_months_later = now + Duration::days(180);

        let url = format!("{}/games", RAWG_API_URL);

        let response = self.client
            .get(&url)
            .query(&[
                ("key", &self.config.api.rawg_api_key),
                ("dates", &format!("{},{}", now.format("%Y-%m-%d"), six_months_later.format("%Y-%m-%d"))),
                ("ordering", &"-added".to_string()),
                ("page", &page.to_string()),
                ("page_size", &"20".to_string()),
            ])
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }
}
