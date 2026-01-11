use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::{error, info, instrument};

use crate::core::models::{AppState, CreateTodo, Todo};
use crate::core::db;

/// Get all todos
#[instrument(skip(state))]
pub async fn get_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    match db::get_todos(&state.db).await {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => {
            error!(error = %e, "Failed to fetch todos from database");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new todo
#[instrument(skip(state), fields(title = %todo.title, description = %todo.description, title_length = todo.title.len()))]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<CreateTodo>,
) -> Result<StatusCode, StatusCode> {
    // Validate title length
    if todo.title.len() > 120 {
        error!(
            title = %todo.title,
            description = %todo.description,
            title_length = todo.title.len(),
            max_length = 120,
            "Todo title exceeds maximum length"
        );
        return Err(StatusCode::BAD_REQUEST);
    }

    match db::create_todo(&state.db, &todo).await {
        Ok(_) => {
            info!(title = %todo.title, description = %todo.description, "Todo created successfully");
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            error!(error = %e, title = %todo.title, description = %todo.description, "Failed to create todo");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
