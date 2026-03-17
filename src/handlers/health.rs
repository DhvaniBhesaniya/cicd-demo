use axum::response::Json;

use crate::models::HealthResponse;

/// GET /health — used by Docker & Render health checks
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
