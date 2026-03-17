use axum::{
    Router,
    routing::get,
};
use tower_http::cors::CorsLayer;

use crate::handlers;

/// Build the application router with all routes and middleware.
pub fn app() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route("/todos/:id", get(handlers::get_todo))
        .layer(CorsLayer::permissive()) // allow all origins (demo only)
}
