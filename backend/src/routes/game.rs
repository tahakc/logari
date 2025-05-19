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
