use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use anyhow::Result;

use crate::core::models::{AppState, Todo};

/// Get all todos
pub async fn get_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    dbg!(&state);
    match tokio::fs::read_to_string(&state.todos_path).await {
        Ok(data) => {
            let todos: Vec<Todo> = serde_json::from_str(&data).map_err(|e| {
                eprintln!("Failed to parse todos JSON: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            Ok(Json(todos))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // File doesn't exist yet, return empty array
            Ok(Json(Vec::new()))
        }
        Err(e) => {
            eprintln!("Failed to read todos file: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new todo
pub async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<Todo>,
) -> Result<StatusCode, StatusCode> {
    dbg!(&todo);
    // Read existing todos or create empty vector if file doesn't exist
    let data = match tokio::fs::read_to_string(&state.todos_path).await {
        Ok(d) => d,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "[]".to_string(),
        Err(e) => {
            eprintln!("Failed to read todos file: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Parse existing todos
    let mut todos: Vec<Todo> = serde_json::from_str(&data).map_err(|e| {
        eprintln!("Failed to parse todos JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Add new todo
    todos.push(todo);

    // Serialize and write back to file
    let json_data = serde_json::to_string_pretty(&todos).map_err(|e| {
        eprintln!("Failed to serialize todos: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // If file does not exist, it will be created
    tokio::fs::write(&state.todos_path, json_data)
        .await
        .map_err(|e| {
            eprintln!("Failed to write todos file: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::CREATED)
}
