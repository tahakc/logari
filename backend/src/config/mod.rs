use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

// Will add jwt validation later
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub supabase_url: Option<String>,
    pub supabase_jwt_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        // load if .env exists
        dotenv.ok();

        let host = env::var("HOST").unwrap_or_else(|_|, "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_|, "8080".to_string())
            .parse()
            .expect("PORT must be a numbver");
        
        // Wil add jwt validation later
        let supabase_url = env::var("SUPABASE_URL").ok();
        let supabase_jwt_secret = env::var("SUPABASE_JWT_SECRET").ok();

        Ok(Config {
            server: ServerConfig { host, port },
            auth: AuthConfig {
                supabase_url,
                supabase_jwt_secret,
            }
        })
    }
}


