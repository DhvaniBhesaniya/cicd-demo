use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ─── Data Models ────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

// ─── Handlers ───────────────────────────────────────────────────────────────

// GET /
async fn root() -> Json<serde_json::Value> {
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

// GET /health  — used by Docker & Render health checks
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// GET /todos
async fn list_todos() -> Json<Vec<Todo>> {
    // In a real app you'd query a database; here we return static data.
    Json(vec![
        Todo { id: 1, title: "Learn Rust".to_string(),        done: true  },
        Todo { id: 2, title: "Build with Axum".to_string(),   done: true  },
        Todo { id: 3, title: "Deploy to Render".to_string(),  done: false },
    ])
}

// GET /todos/:id
async fn get_todo(Path(id): Path<u32>) -> Result<Json<Todo>, StatusCode> {
    // Simulated lookup
    if id == 1 {
        Ok(Json(Todo { id: 1, title: "Learn Rust".to_string(), done: true }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// POST /todos
async fn create_todo(Json(payload): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    let new_todo = Todo {
        id: 42,               // In reality: auto-increment from DB
        title: payload.title,
        done: false,
    };
    (StatusCode::CREATED, Json(new_todo))
}

// ─── Router ─────────────────────────────────────────────────────────────────

fn app() -> Router {
    Router::new()
        .route("/",           get(root))
        .route("/health",     get(health))
        .route("/todos",      get(list_todos).post(create_todo))
        .route("/todos/:id",  get(get_todo))
        .layer(CorsLayer::permissive()) // allow all origins (demo only)
}

// ─── Entry point ────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // Set up structured logging  (RUST_LOG=debug cargo run)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Render injects PORT at runtime; fallback to 3000 locally
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}