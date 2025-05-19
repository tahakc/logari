use axum::{
    extract::{Path, State, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::Result;
use crate::external::ApiClient;
use crate::models::search::{MediaType, SearchResponse};

#[derive(Debug, Deserialize)]
pub struct GameSearchRequest {
    pub query: String,
    pub page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
}

pub async fn search_game(
    State(api_client): State<ApiClient>,
    Json(request): Json<GameSearchRequest>,
) -> Result<Json<SearchResponse>> {
    let page = request.page.unwrap_or(1);
    let results = api_client.search_rawg(&request.query, page).await?;
    Ok(Json(results))
}

pub async fn get_game_details(
    State(api_client): State<ApiClient>,
    Path(game_id): Path<u32>,
) -> Result<Json<serde_json::Value>> {
    let game_details = api_client.get_game_details(game_id).await?;
    Ok(Json(game_details))
}

pub async fn get_popular_games(
    State(api_client): State<ApiClient>,
    Query(params): Query<PageQuery>,
) -> Result<Json<serde_json::Value>> {
    let page = params.page.unwrap_or(1);
    let popular_games = api_client.get_popular_games(page).await?;
    Ok(Json(popular_games))
}
