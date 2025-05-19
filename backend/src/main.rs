mod config;
mod routes;
mod external;

use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use std::sync::Arc;

use tower_http::trace::{self, TraceLayer};
use tower_http::cors::{CorsLayer, Any};
use tracing::Level;

use config::Config;
use routes::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = match Config::from_env() {
        Ok(cfg) => Arc::new(cfg),
        Err(err) => {
            tracing::error!("Failed to load configuration: {}", err);
            std::process::exit(1);
        }
    };

    let host = IpAddr::from_str(&config.server.host).unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let addr = SocketAddr::from((host, config.server.port));

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any); // TODO: Make this more restrictive later
    
    let app = create_router(config)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors);

    tracing::info!("Starting server at {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app,
    )
    .await?;

    Ok(())
}
