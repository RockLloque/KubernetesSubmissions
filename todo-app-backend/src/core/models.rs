use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
// TODO: add an order for Todos. space by 10_000
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub todos_path: String,
}
