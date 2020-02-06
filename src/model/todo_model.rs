use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoModel {
    pub id: u64,
    pub name: String,
}
