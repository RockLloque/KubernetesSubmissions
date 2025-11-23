use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub todos_path: String,
}
