#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub description: String,
}
