use serde::Deserialize;
use std::env;
use tracing::info;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub buddha_endpoint: String,
    pub discord_webhook_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new() -> AppResult<Self> {
        dotenv::dotenv().ok();

        let config = Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "9001".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("Invalid SERVER_PORT".to_string()))?,
            buddha_endpoint: env::var("BUDDHA_ENDPOINT").map_err(|_| {
                AppError::ConfigError(
                    "BUDDHA_ENDPOINT environment variable is required".to_string(),
                )
            })?,
            discord_webhook_url: env::var("DISCORD_WEBHOOK_URL").map_err(|_| {
                AppError::ConfigError(
                    "DISCORD_WEBHOOK_URL environment variable is required".to_string(),
                )
            })?,
            api_key: env::var("API_KEY").map_err(|_| {
                AppError::ConfigError("API_KEY environment variable is required".to_string())
            })?,
        };

        info!("Configuration loaded successfully");
        Ok(config)
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_addr() {
        let config = Config {
            server_host: "127.0.0.1".to_string(),
            server_port: 9001,
            buddha_endpoint: "http://example.com".to_string(),
            discord_webhook_url: "https://discord.com/api/webhooks/test".to_string(),
            api_key: "api-key".to_string(),
        };

        assert_eq!(config.server_addr(), "127.0.0.1:3030");
    }
}
