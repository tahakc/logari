pub mod health;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::config::Config;
use crate::external::ApiClient;

pub fn create_router(config: Arc<Config>) -> Router {
    let api_client = ApiClient::new(config.clone());

    let health_routes = Router::new()
        .route("/health", get(health::health_check))
        .with_state(config.clone());

    Router::new()
        .nest("/api/v1",
            health_routes
        )
}
