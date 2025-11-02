use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub description: String,
}
