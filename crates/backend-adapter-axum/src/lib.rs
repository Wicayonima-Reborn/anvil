use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use backend_core::health::HealthState;

/// Build health routes for Axum.
///
/// This adapter maps core health state to HTTP endpoints.
/// No business logic or state is owned by this adapter.
pub fn health_routes(health: Arc<HealthState>) -> Router {
    Router::new()
        .route("/health/live", get({
            let health = Arc::clone(&health);
            move || async move {
                if health.is_live() {
                    StatusCode::OK.into_response()
                } else {
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
            }
        }))
        .route("/health/ready", get({
            let health = Arc::clone(&health);
            move || async move {
                if health.is_ready() {
                    StatusCode::OK.into_response()
                } else {
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
            }
        }))
        .route("/health/ready/reasons", get({
            let health = Arc::clone(&health);
            move || async move {
                let reasons = health.degradation_reasons();
                Json(
                    reasons
                        .into_iter()
                        .map(|r| r.code)
                        .collect::<Vec<_>>(),
                )
            }
        }))
}