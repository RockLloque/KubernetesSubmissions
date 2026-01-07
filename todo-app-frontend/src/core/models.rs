use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub sorting_order: i32,
}
