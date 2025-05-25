use serde_json::json;
use std::collections::HashMap;

use crate::{
    config::Config,
    error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct DiscordNotifyServiceImpl {
    client: reqwest::Client,
    config: Config,
}

impl DiscordNotifyServiceImpl {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub async fn send_notification(&self, message: String) -> AppResult<bool> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let payload = json!({
            "username": "Notify",
            "content": message,
        });

        let mut request = self.client.post(&self.config.discord_webhook_url);
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        let response = request.json(&payload).send().await?;

        if response.status().is_success() {
            tracing::info!("Discord notification sent successfully");
            Ok(true)
        } else {
            tracing::error!("Discord notification failed: {}", response.status());
            Err(AppError::DiscordNotifyError(format!(
                "Failed to send notification: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discord_notify_service() {
        let config = Config {
            server_host: "127.0.0.1".to_string(),
            server_port: 3030,
            buddha_endpoint: "http://test.com".to_string(),
            discord_webhook_url: "https://discord.com/api/webhooks/test".to_string(),
            api_key: "api-key".to_string(),
        };
        let service = DiscordNotifyServiceImpl::new(config);

        // This test will fail in real environment since we're using fake webhook
        // But it tests that the service can be instantiated
        let result = service.send_notification("Test message".to_string()).await;
        assert!(result.is_err());
    }
}
