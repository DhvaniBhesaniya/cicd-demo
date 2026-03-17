use axum::response::Json;

/// GET / — welcome message with available endpoints
pub async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Welcome to Rust + Axum + Tokio API!",
        "endpoints": [
            "GET  /health",
            "GET  /todos",
            "GET  /todos/:id",
            "POST /todos",
        ]
    }))
}
