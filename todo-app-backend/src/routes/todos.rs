use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::core::models::{AppState, CreateTodo, Todo};
use crate::core::db;

/// Get all todos
pub async fn get_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    match db::get_todos(&state.db).await {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new todo
pub async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<CreateTodo>,
) -> Result<StatusCode, StatusCode> {
    match db::create_todo(&state.db, &todo).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
