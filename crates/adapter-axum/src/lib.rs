use std::sync::Arc;

use axum::{routing::get, Router};
use backend_core::health::HealthState;

/// Build health routes for Axum.
///
/// This adapter maps core health state to HTTP endpoints.
/// No business logic is implemented here.
pub fn health_routes(health: Arc<HealthState>) -> Router {
    Router::new()
        .route("/health/live", get({
            let health = Arc::clone(&health);
            move || async move {
                if health.is_live() {
                    "OK"
                } else {
                    "NOT_LIVE"
                }
            }
        }))
        .route("/health/ready", get({
            let health = Arc::clone(&health);
            move || async move {
                if health.is_ready() {
                    "READY"
                } else {
                    "NOT_READY"
                }
            }
        }))
}