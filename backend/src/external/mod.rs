use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;

#[derive(Clone)]
pub struct ApiClient {
    pub client: Client,
    pub config: Arc<Config>,
}

impl ApiClient {
    pub fn new(config: Arc<Config>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }
}
