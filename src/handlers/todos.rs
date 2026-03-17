use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};

use crate::models::{CreateTodo, Todo};

/// GET /todos — return all todos
pub async fn list_todos() -> Json<Vec<Todo>> {
    // In a real app you'd query a database; here we return static data.
    Json(vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            done: true,
        },
        Todo {
            id: 2,
            title: "Build with Axum".to_string(),
            done: true,
        },
        Todo {
            id: 3,
            title: "Deploy to Render".to_string(),
            done: false,
        },
    ])
}

/// GET /todos/:id — return a single todo by ID
pub async fn get_todo(Path(id): Path<u32>) -> Result<Json<Todo>, StatusCode> {
    // Simulated lookup
    if id == 1 {
        Ok(Json(Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            done: true,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// POST /todos — create a new todo
pub async fn create_todo(Json(payload): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    let new_todo = Todo {
        id: 42, // In reality: auto-increment from DB
        title: payload.title,
        done: false,
    };
    (StatusCode::CREATED, Json(new_todo))
}
