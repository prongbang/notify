use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    error::{AppError, AppResult},
    models::{BuddhaDate, QueryParams},
    services::AppState,
};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[axum::debug_handler]
pub async fn notify_handler(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> AppResult<impl IntoResponse> {
    // Validate API key
    if params.key != state.config.api_key {
        return Err(AppError::AuthenticationError("Invalid API key".into()));
    }

    // Get Buddha calendar information
    let buddha_date = BuddhaDate::from_now();
    let buddha = state.buddha_service.get_buddha(buddha_date).await?;

    // Send notification if it's a Buddha day
    if buddha.today.found {
        state
            .notify_service
            .send_notification(buddha.today.description)
            .await?;
        Ok(StatusCode::OK)
    } else if buddha.tomorrow.found {
        state
            .notify_service
            .send_notification(buddha.tomorrow.description)
            .await?;
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, services};
    use axum::{body::Body, http::Request, routing::get, Router};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_notify_handler_invalid_key() {
        let app = setup_test_app().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/notify?key=invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    async fn setup_test_app() -> Router {
        let config = Config {
            server_host: "127.0.0.1".to_string(),
            server_port: 9001,
            buddha_endpoint: "http://test.com".to_string(),
            discord_webhook_url: "http://test.com".to_string(),
            api_key: "api-key".to_string(),
        };

        let state = AppState::new(
            config.clone(),
            services::buddha::BuddhaServiceImpl::new(config.clone()),
            services::discord::DiscordNotifyServiceImpl::new(config),
        );

        Router::new()
            .route("/notify", get(notify_handler))
            .with_state(state)
    }
}
