use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordManufacturer {
    pub id: i64,
    pub name: String,
}

impl Default for RecordManufacturer {
    fn default() -> Self {
        Self {
            id: 0,
            name: "other".to_string(),
        }
    }
}
