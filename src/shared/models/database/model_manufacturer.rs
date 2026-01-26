use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordManufacturer {
    pub id: i64,
    pub name: String,
}
