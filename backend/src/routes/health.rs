use axum::{
    extract::State,
    Json
};
use serde_json::json;
use std::sync::Arc;

use crate::config::Config;

pub async fn health_check(State(_config): State<Arc<Config>>) -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "service": env!("CARGO_PKG_NAME")
    }))
}
