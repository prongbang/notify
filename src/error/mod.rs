use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),

    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Discord service error: {0}")]
    DiscordNotifyError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            Self::HttpClientError(_) => StatusCode::BAD_REQUEST,
            Self::CsvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CacheError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DiscordNotifyError(_) => StatusCode::BAD_GATEWAY,
            Self::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_type(&self) -> &'static str {
        match self {
            Self::AuthenticationError(_) => "AUTHENTICATION_ERROR",
            Self::HttpClientError(_) => "HTTP_CLIENT_ERROR",
            Self::CsvError(_) => "CSV_ERROR",
            Self::CacheError(_) => "CACHE_ERROR",
            Self::DiscordNotifyError(_) => "DISCORD_NOTIFY_ERROR",
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::InternalError(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = json!({
            "error": {
                "type": self.error_type(),
                "message": self.to_string()
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_error_conversion() {
        let error = AppError::AuthenticationError("Invalid token".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_error_json_response() {
        let error = AppError::DiscordNotifyError("Failed to send".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    }
}
