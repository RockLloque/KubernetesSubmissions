use gloo_net::http::Request;
use crate::core::models::{Todo, CreateTodo};

const API_BASE_URL: &str = "/todos";

pub async fn fetch_todos() -> Result<Vec<Todo>, String> {
    let response = Request::get(API_BASE_URL)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch todos: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json::<Vec<Todo>>()
        .await
        .map_err(|e| format!("Failed to parse todos: {}", e))
}

pub async fn create_todo(todo: CreateTodo) -> Result<(), String> {
    let response = Request::post(API_BASE_URL)
        .json(&todo)
        .map_err(|e| format!("Failed to serialize todo: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to create todo: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    Ok(())
}
